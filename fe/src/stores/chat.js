import { defineStore } from 'pinia'
import { nextTick, reactive, ref } from 'vue'
import ChatMessage from './models/chatMessage'
import Connection from './models/connection'
import ConnectionStatus from './models/connectionStatus'

export const useChatStore = defineStore('chat', () => {
    let ws = ref()
    let messages = reactive([])
    let reconnectTimeout = null
    let user_id = ref('')
    const maxReconnectAttempts = 10
    let reconnectAttempts = 0
    let connections = reactive({ currentChat: [], otherSides: [] })
    let connectionStatus = ref(ConnectionStatus.INACTIVE)

    function getPath(chat_uuid) {
        return chat_uuid ? `4/ws/${chat_uuid}` : '5/ws'
    }

    function connect(chat_uuid) {
        if (ws.value && ws.value.readyState === WebSocket.OPEN) {
            return
        }

        // Todo: make this localhost dynamic
        ws.value = new WebSocket(`ws://localhost:912${getPath(chat_uuid)}`)

        // Todo: put this in a separate function
        ws.value.onmessage = async (event) => {
            const message = JSON.parse(event.data)
            // todo: handle other message types as well (ping/pong connect/disconnect)
            //todo:  add user id to temp message

            if ('Connection' in message.data) {
                console.log('got new connection', message.data.Connection.status)
                const connection = new Connection(message.data.Connection.status, message.data.Connection.user_id)

                const target =
                    message.data.Connection.user_id === user_id.value ? connections.currentChat : connections.otherSides

                // Don't push multiple times
                if (
                    !target.some((conn) => conn.user_id === connection.user_id) &&
                    (connection.status === ConnectionStatus.CONNECTED ||
                        connection.status === ConnectionStatus.STAYING_ALIVE)
                ) {
                    target.push(connection)

                    // If we push a new item to the otherSides array, we need to send a connection message (staying alive) as
                    // well, so the other side knows we are already connected
                    if (
                        connection.user_id !== user_id.value &&
                        connections.otherSides.some((conn) => conn.user_id === connection.user_id)
                    ) {
                        sendConnection(user_id, ConnectionStatus.STAYING_ALIVE)
                        console.log('sending staying alive')
                    }
                } else if (
                    target.some((conn) => conn.user_id === connection.user_id) &&
                    connection.status === ConnectionStatus.DISCONNECTED
                ) {
                    const index = target.findIndex((conn) => conn.user_id === connection.user_id)
                    if (index !== -1) {
                        target.splice(index, 1)
                    }
                }

                let activeCounter = 0
                connections.otherSides.forEach((c) => {
                    if (c.status === ConnectionStatus.CONNECTED || c.status === ConnectionStatus.STAYING_ALIVE) {
                        activeCounter++
                    }
                })

                if (activeCounter > 0) {
                    connectionStatus.value = ConnectionStatus.ACTIVE
                } else {
                    connectionStatus.value = ConnectionStatus.INACTIVE
                }
            } else if ('ChatMessage' in message.data) {
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
            } else {
                // todo: handle ping
                console.error('Unknown message type:', message)
            }

            await nextTick()
        }

        ws.value.onopen = () => {
            console.log('WebSocket connection opened')
            sendConnection(user_id, ConnectionStatus.CONNECTED)

            reconnectAttempts = 0
        }

        ws.value.onclose = () => {
            console.log('WebSocket connection closed from server')
            attemptReconnect(chat_uuid)
        }

        ws.value.onerror = (error) => {
            console.error('WebSocket error (disconnect?):', error)
        }
    }

    function disconnect() {
        if (ws.value) {
            console.log('Sending disconnect message')
            sendConnection(user_id, ConnectionStatus.DISCONNECTED)

            ws.value.onclose = null
            console.log('WebSocket connection closed')
            ws.value.close()
            ws.value = null
        }

        if (reconnectTimeout) {
            clearTimeout(reconnectTimeout)
            reconnectTimeout = null
        }

        this.connectionStatus = ConnectionStatus.INACTIVE
        this.messages = []
        this.user_id = ''
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

    return {
        messages,
        user_id,
        connections,
        connectionStatus,
        connect,
        disconnect,
        sendMessage,
    }
})

function handleWsMessage(event) {
    const message = JSON.parse(event.data)
}
