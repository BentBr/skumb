import { defineStore } from 'pinia'
import { computed, nextTick, reactive, ref } from 'vue'
import ChatMessage from './models/chatMessage'
import Connection from './models/connection'
import ConnectionStatusConstant from './models/connectionStatusConstant'
import router from '../router'

export const useChatStore = defineStore('chat', () => {
    let ws = ref()
    let messages = reactive([])
    let reconnectTimeout = null
    let user_id = ref('')
    let chat_uuid = ref('')
    const maxReconnectAttempts = 10
    let reconnectAttempts = 0
    let connections = reactive({ currentChat: [], otherSides: [] })
    let connectionStatus = ref(ConnectionStatusConstant.INACTIVE)
    const usernameEntered = ref(false)

    function getChatWsPath() {
        return `${import.meta.env.VITE_WS_PROTOCOL}://${import.meta.env.VITE_API_URL}/ws/${chat_uuid.value}`
    }

    function getChatUuidApiPath() {
        return `${import.meta.env.VITE_HTTP_PROTOCOL}://${import.meta.env.VITE_API_URL}/v1/chat/uuid`
    }

    function getChatPath() {
        return `${import.meta.env.VITE_HTTP_PROTOCOL}://${import.meta.env.VITE_URL}/chat/${chat_uuid.value}`
    }

    function connect() {
        if (ws.value && ws.value.readyState === WebSocket.OPEN) {
            return
        }

        ws.value = new WebSocket(getChatWsPath())

        ws.value.onmessage = async (event) => {
            handleWsMessage(event)

            await nextTick()
        }

        ws.value.onopen = () => {
            console.log('WebSocket connection opened')
            sendConnection(user_id, ConnectionStatusConstant.CONNECTED)

            reconnectAttempts = 0
        }

        ws.value.onclose = () => {
            console.log('WebSocket connection closed from server')
            attemptReconnect(chat_uuid.value)
        }

        ws.value.onerror = (error) => {
            console.error('WebSocket error (disconnect?):', error)
        }
    }

    function disconnect() {
        if (ws.value) {
            console.log('Sending disconnect message')
            sendConnection(user_id, ConnectionStatusConstant.DISCONNECTED)

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
        user_id.value = ''
        chat_uuid.value = ''
    }

    function attemptReconnect(chat_uuid) {
        if (reconnectAttempts < maxReconnectAttempts) {
            reconnectAttempts++

            const timeout = Math.min(1000 * 2 ** reconnectAttempts, 30000)
            reconnectTimeout = setTimeout(() => connect(chat_uuid), timeout)
        }
    }

    function sendMessage(user_id, text) {
        if (ws.value && ws.value.readyState === WebSocket.OPEN) {
            const chatMessage = new ChatMessage(user_id, text)
            const wsMessage = { data: { ChatMessage: chatMessage } }

            // Add the message to the messages array immediately with temporary values
            //todo:  add user id to temp message
            messages.push(chatMessage)
            ws.value.send(JSON.stringify(wsMessage))
        }
    }

    function sendConnection(user_id, status) {
        if (ws.value && ws.value.readyState === WebSocket.OPEN) {
            const connectionMessage = new Connection(status, user_id).toPlainObject()
            const wsMessage = { data: { Connection: connectionMessage } }

            ws.value.send(JSON.stringify(wsMessage))
        }
    }

    function handleWsMessage(event) {
        const message = JSON.parse(event.data)
        // todo: handle other message types as well (ping/pong connect/disconnect)
        //todo:  add user id to temp message

        if ('Connection' in message.data) {
            handleConnectionMessage(message)
        } else if ('ChatMessage' in message.data) {
            handleChatMessage(message)
        } else {
            // todo: handle ping
            console.error('Unknown message type:', message)
        }
    }

    function handleConnectionMessage(message) {
        console.log('got new connection', message.data.Connection.status)
        const connection = new Connection(message.data.Connection.status, message.data.Connection.user_id)

        const target =
            message.data.Connection.user_id === user_id.value ? connections.currentChat : connections.otherSides

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
                connection.user_id !== user_id.value &&
                connections.otherSides.some((conn) => conn.user_id === connection.user_id)
            ) {
                sendConnection(user_id, ConnectionStatusConstant.STAYING_ALIVE)
                console.log('sending staying alive')
            }
        } else if (
            target.some((conn) => conn.user_id === connection.user_id) &&
            connection.status === ConnectionStatusConstant.DISCONNECTED
        ) {
            const index = target.findIndex((conn) => conn.user_id === connection.user_id)
            if (index !== -1) {
                target.splice(index, 1)
            }
        }

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

    function handleChatMessage(message) {
        const chatMessage = new ChatMessage(
            message.data.ChatMessage.user_id,
            message.data.ChatMessage.text,
            message.data.ChatMessage.uuid,
            message.data.ChatMessage.message_sent_at,
        )

        const index = messages.findIndex(
            (m) => m.user_id === chatMessage.user_id && m.text === chatMessage.text && m.uuid === 'temp',
        )
        if (index !== -1) {
            // Update the existing message with the full data from the server
            messages[index] = chatMessage
        } else {
            // Add the new message to the array
            messages.push(chatMessage)
        }
    }

    const fetchChatUuid = async () => {
        try {
            const response = await fetch(getChatUuidApiPath())
            if (!response.ok) {
                console.error('Network response was not ok for uuid fetching: ', response.status)
            }

            const data = await response.json()
            console.log('Fetched chat UUID:', data.data)
            chat_uuid.value = data.data

            await router.push(`/chat/${chat_uuid.value}`)
        } catch (error) {
            console.error('Failed to fetch chat UUID:', error)
        }
    }

    const isActive = computed(() => {
        return connectionStatus.value === ConnectionStatusConstant.ACTIVE
    })

    return {
        messages,
        user_id,
        chat_uuid,
        connections,
        connectionStatus,
        isActive,
        usernameEntered,
        connect,
        disconnect,
        sendMessage,
        fetchChatUuid,
        getChatPath,
    }
})
