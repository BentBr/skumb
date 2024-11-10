import { defineStore } from 'pinia'
import { computed, nextTick, reactive, ref } from 'vue'
import ChatMessage from './models/chatMessage'
import Connection from './models/connection'
import ConnectionStatusConstant from './models/connectionStatusConstant'
import router from '../router'
import {
    decryptMessageFromBase64Representation,
    deriveSymmetricKey,
    encryptMessageBase64Representation,
    exportKeyToJwk,
    exportSymmetricKeyToRaw,
    importSymmetricKeyFromRaw,
    generateKeyPair,
    importKeyFromJwk,
} from '../security/security.js'
import GroupKey from './models/groupKey.js'
import { useI18n } from 'vue-i18n'

export const useChatStore = defineStore('chat', () => {
    let ws = ref()
    let messages = reactive([])
    let reconnectTimeout = null
    let userId = ref('')
    let userName = ref('')
    let chatId = ref('')
    const maxReconnectAttempts = 10
    let reconnectAttempts = 0
    let connections = reactive({ currentChat: [], otherSides: [] }) // Arrays of Connection objects
    let connectionStatus = ref(ConnectionStatusConstant.INACTIVE) // Enum
    const usernameEntered = ref(false) // bool
    const privateKey = ref(null) // CryptoKey object
    const publicKey = ref(null) // CryptoKey object
    let groupKeyCreationDate = ref(null) // String
    let groupKeyDerivedSymmetric = ref(null) // CryptoKey object
    const { t } = useI18n()

    function getChatWsPath() {
        return `${import.meta.env.VITE_WS_PROTOCOL}://${import.meta.env.VITE_API_URL}/ws/${chatId.value}`
    }

    function getChatUuidApiPath() {
        return `${import.meta.env.VITE_HTTP_PROTOCOL}://${import.meta.env.VITE_API_URL}/v1/chat/uuid`
    }

    function getChatPath() {
        return `${import.meta.env.VITE_HTTP_PROTOCOL}://${import.meta.env.VITE_URL}/chat/${chatId.value}`
    }

    function getUserUuidApiPath() {
        return `${import.meta.env.VITE_HTTP_PROTOCOL}://${import.meta.env.VITE_API_URL}/v1/user/uuid`
    }

    async function connect() {
        if (ws.value && ws.value.readyState === WebSocket.OPEN) {
            if (userId.value === '') {
                await setNewUserId()
            }

            return
        }

        // Initializing those keys here, because we need to have them before connecting
        if (!privateKey.value || !publicKey.value) {
            await generateAsymmetricKeys()
            console.log('Init security done')
        }

        if (!userId.value) {
            await setNewUserId()
        }

        ws.value = new WebSocket(getChatWsPath())

        ws.value.onmessage = async (event) => {
            await handleWsMessage(event)

            await nextTick()
        }

        ws.value.onopen = () => {
            console.log('WebSocket connection opened')
            sendConnection(ConnectionStatusConstant.CONNECTED)

            reconnectAttempts = 0
        }

        ws.value.onclose = () => {
            console.log('WebSocket connection closed from server')
            attemptReconnect(chatId.value)
        }

        ws.value.onerror = (error) => {
            console.error('WebSocket error (disconnect?):', error)
        }
    }

    async function disconnect() {
        if (ws.value) {
            console.log('Sending disconnect message')
            await sendConnection(ConnectionStatusConstant.DISCONNECTED)

            ws.value.onclose = null
            console.log('WebSocket connection closed')
            ws.value.close()
            ws.value = null
        }

        if (reconnectTimeout) {
            clearTimeout(reconnectTimeout)
            reconnectTimeout = null
        }

        connectionStatus.value = ConnectionStatusConstant.INACTIVE
        messages = []
        userId.value = ''
        chatId.value = ''
        publicKey.value = null
        privateKey.value = null
    }

    function attemptReconnect(chat_uuid) {
        if (reconnectAttempts < maxReconnectAttempts) {
            reconnectAttempts++

            const timeout = Math.min(1000 * 2 ** reconnectAttempts, 30000)
            reconnectTimeout = setTimeout(() => connect(chat_uuid), timeout)
        }
    }

    async function sendMessageToUser(text) {
        if (ws.value && ws.value.readyState === WebSocket.OPEN) {
            const { cipher, iv } = await encryptMessageBase64Representation(groupKeyDerivedSymmetric.value, text)
            const chatMessage = new ChatMessage(userId.value, cipher, iv)
            const wsMessage = { data: { ChatMessage: chatMessage } }

            ws.value.send(JSON.stringify(wsMessage))
        }
    }

    async function sendConnection(status) {
        if (ws.value && ws.value.readyState === WebSocket.OPEN) {
            const connectionMessage = new Connection(
                status,
                userId.value,
                userName.value,
                await exportKeyToJwk(publicKey.value),
            ).toPlainObject()
            const wsMessage = { data: { Connection: connectionMessage } }

            ws.value.send(JSON.stringify(wsMessage))
        }
    }

    async function sendGroupKey(forUserId) {
        if (ws.value && ws.value.readyState === WebSocket.OPEN) {
            // Getting the public key of the target user
            const targetUser = connections.otherSides.find((connection) => connection.user_id === forUserId)
            if (!targetUser) {
                console.error('No connection found for user:', forUserId)
                return
            }
            const derivedEncryptionKey = await deriveSymmetricKey(privateKey.value, targetUser.public_key)
            const encryptedGroupKey = await encryptMessageBase64Representation(
                derivedEncryptionKey,
                await exportSymmetricKeyToRaw(groupKeyDerivedSymmetric.value),
            )

            const groupKeyMessage = new GroupKey(
                encryptedGroupKey.cipher,
                encryptedGroupKey.iv,
                groupKeyCreationDate.value,
                forUserId,
                userId.value,
            ).toPlainObject()
            const wsMessage = { data: { GroupKey: groupKeyMessage } }

            ws.value.send(JSON.stringify(wsMessage))
        }
    }

    async function handleWsMessage(event) {
        const message = JSON.parse(event.data)
        // todo: handle other message types as well (ping/pong connect/disconnect)
        if ('Connection' in message.data) {
            await handleConnectionMessage(message)
        } else if ('ChatMessage' in message.data) {
            await handleChatMessage(message)
        } else if ('GroupKey' in message.data) {
            await handleGroupKeyMessage(message)
        } else {
            // todo: handle ping
            console.error('Unknown message type:', message)
        }
    }

    async function handleConnectionMessage(message) {
        console.log('got new connection', message.data.Connection.status)

        const connection = new Connection(
            message.data.Connection.status,
            message.data.Connection.user_id,
            message.data.Connection.user_name,
            await importKeyFromJwk(message.data.Connection.public_key),
        )

        const target =
            message.data.Connection.user_id === userId.value ? connections.currentChat : connections.otherSides

        // Don't push multiple times
        if (
            !target.some((conn) => conn.user_id === connection.user_id) &&
            (connection.status === ConnectionStatusConstant.CONNECTED ||
                connection.status === ConnectionStatusConstant.STAYING_ALIVE)
        ) {
            target.push(connection)

            // If we push a new item to the otherSides array, we need to send a connection message (staying alive) as
            // well, so the other side knows we are already connected
            if (
                connection.user_id !== userId.value &&
                connections.otherSides.some((conn) => conn.user_id === connection.user_id)
            ) {
                await sendConnection(ConnectionStatusConstant.STAYING_ALIVE)
                await sendGroupKey(connection.user_id)
            }
            // Remove connection if disconnecting
        } else if (
            target.some((conn) => conn.userId === connection.user_id) &&
            connection.status === ConnectionStatusConstant.DISCONNECTED
        ) {
            const index = target.findIndex((conn) => conn.user_id === connection.user_id)
            if (index !== -1) {
                target.splice(index, 1)
            }
        }

        // Checking if connection is ours, and we don't have a group key yet -> create one
        if (connection.user_id === userId.value && !groupKeyDerivedSymmetric.value) {
            await createGroupKey()
        }

        // Checking for activity
        let activeCounter = 0
        connections.otherSides.forEach((c) => {
            if (
                c.status === ConnectionStatusConstant.CONNECTED ||
                c.status === ConnectionStatusConstant.STAYING_ALIVE
            ) {
                activeCounter++
            }
        })

        if (activeCounter > 0) {
            connectionStatus.value = ConnectionStatusConstant.ACTIVE
        } else {
            connectionStatus.value = ConnectionStatusConstant.INACTIVE
        }
    }

    async function handleChatMessage(message) {
        const decryptedMessage = await decryptMessage(message.data.ChatMessage)
        const chatMessage = new ChatMessage(
            message.data.ChatMessage.user_id,
            decryptedMessage,
            message.data.ChatMessage.iv,
            message.data.ChatMessage.uuid,
            message.data.ChatMessage.message_sent_at,
        )

        const index = messages.findIndex(
            (m) => m.user_id === chatMessage.user_id && m.cipher === chatMessage.cipher && m.user_id === 'temp',
        )
        if (index !== -1) {
            // Update the existing message with the full data from the server
            messages[index] = chatMessage
        } else {
            // Add the new message to the array
            messages.push(chatMessage)
        }
    }

    async function handleGroupKeyMessage(message) {
        // If the message is not meant for this user, ignore it
        if (message.data.GroupKey.for_user_id !== userId.value) {
            return
        }

        // If the current group key is set and older than the new one, ignore it
        const currentKeyDate = new Date(groupKeyCreationDate.value)
        const newKeyDate = new Date(message.data.GroupKey.creation_date)
        if (groupKeyCreationDate.value && currentKeyDate.getTime() < newKeyDate.getTime()) {
            // Sending current one to have the correct key synced
            await sendGroupKey(message.data.GroupKey.from_user_id)

            return
        }

        // Only check for this current user
        const publicKeyConnection = connections.otherSides.find(
            (connection) => connection.user_id === message.data.GroupKey.from_user_id,
        )
        if (!publicKeyConnection) {
            console.error(
                'No public key found from sending user for group key decryption:',
                message.data.GroupKey.from_user_id,
            )

            // Let's try sync again
            await sendConnection(ConnectionStatusConstant.STAYING_ALIVE)
            await sendGroupKey(message.data.GroupKey.from_user_id)
            return
        }

        const derivedDecryptionKey = await deriveSymmetricKey(privateKey.value, publicKeyConnection.public_key)
        const decryptedKey = await decryptMessageFromBase64Representation(
            derivedDecryptionKey,
            message.data.GroupKey.encrypted_key,
            message.data.GroupKey.iv,
            t,
        )
        groupKeyDerivedSymmetric.value = await importSymmetricKeyFromRaw(decryptedKey)
        groupKeyCreationDate.value = message.data.GroupKey.creation_date
    }

    const fetchChatUuid = async () => {
        try {
            const response = await fetch(getChatUuidApiPath())
            if (!response.ok) {
                console.error('Network response was not ok for chat uuid fetching: ', response.status)
            }

            const data = await response.json()
            console.log('Fetched chat UUID:', data.data)
            chatId.value = data.data

            await router.push(`/chat/${chatId.value}`)
        } catch (error) {
            console.error('Failed to fetch chat UUID:', error)
        }
    }

    const isActive = computed(() => {
        return connectionStatus.value === ConnectionStatusConstant.ACTIVE
    })

    const generateAsymmetricKeys = async () => {
        const keyPair = await generateKeyPair()

        privateKey.value = keyPair.privateKey
        publicKey.value = keyPair.publicKey
    }

    const decryptMessage = async (message) => {
        return await decryptMessageFromBase64Representation(
            groupKeyDerivedSymmetric.value,
            message.cipher,
            message.iv,
            t,
        )
    }

    function getUserNameForId(userId) {
        const connection =
            connections.currentChat.find((conn) => conn.user_id === userId) ||
            connections.otherSides.find((conn) => conn.user_id === userId)

        if (connection) {
            return connection.user_name
        } else {
            throw new Error(`No user name found for userId: ${userId}`)
        }
    }

    async function setNewUserId() {
        try {
            const response = await fetch(getUserUuidApiPath())
            if (!response.ok) {
                console.error('Network response was not ok for user uuid fetching: ', response.status)
            }

            const data = await response.json()
            console.log('Fetched user UUID:', data.data)
            userId.value = data.data
        } catch (error) {
            console.error('Failed to fetch user UUID:', error)
        }
    }

    async function createGroupKey() {
        console.log('creating new group key')
        const key = await generateKeyPair()
        groupKeyCreationDate.value = new Date().toISOString().split('.')[0]
        groupKeyDerivedSymmetric.value = await deriveSymmetricKey(key.privateKey, key.publicKey)
    }

    return {
        messages,
        userId,
        userName,
        chatId,
        connections,
        connectionStatus,
        isActive,
        usernameEntered,
        connect,
        disconnect,
        sendMessageToUser,
        fetchChatUuid,
        getChatPath,
        getUserNameForId,
    }
})
