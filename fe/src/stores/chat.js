import { defineStore } from 'pinia';
import {nextTick, reactive, ref} from 'vue'

export const useChatStore = defineStore('chat', () => {
    let ws = ref(null);
    let messages = reactive([]);
    let reconnectTimeout = null;
    const maxReconnectAttempts = 10;
    let reconnectAttempts = 0;

    function getPath(chat_uuid) {
        return chat_uuid ? `4/ws/${chat_uuid}` : '5/ws';
    }

    function connect(chat_uuid) {
        if (ws.value && ws.value.readyState === WebSocket.OPEN) {
            return;
        }

        ws.value = new WebSocket(`ws://localhost:912${getPath(chat_uuid)}`);

        ws.value.onmessage = async (event) => {
            const message = JSON.parse(event.data);
            //todo:  add user id to temp message

            const index = messages.findIndex(m => m.user_uuid === message.user_uuid && m.text === message.text && m.uuid === 'temp');
            if (index !== -1) {
                // Update the existing message with the full data from the server
                messages[index] = message;
            } else {
                // Add the new message to the array
                messages.push(message);
            }

            await nextTick();
        };

        ws.value.onopen = () => {
            console.log('WebSocket connection opened');
            reconnectAttempts = 0;
        };

        ws.value.onclose = () => {
            console.log('WebSocket connection closed from server');
            attemptReconnect(chat_uuid);
        };

        ws.value.onerror = (error) => {
            console.error('WebSocket error:', error);
        };
    }

    function disconnect() {
        if (ws.value) {
            ws.value.onclose = null;
            console.log('WebSocket connection closed');
            ws.value.close();
            ws.value = null;
        }

        if (reconnectTimeout) {
            clearTimeout(reconnectTimeout);
            reconnectTimeout = null;
        }
    }

    function attemptReconnect(chat_uuid) {
        if (reconnectAttempts < maxReconnectAttempts) {
            reconnectAttempts++;
            const timeout = Math.min(1000 * 2 ** reconnectAttempts, 30000);
            reconnectTimeout = setTimeout(() => connect(chat_uuid), timeout);
        }
    }

    function sendMessage(user_uuid, text) {
        if (ws.value && ws.value.readyState === WebSocket.OPEN) {
            const chatMessage = { user_uuid, text };
            // Add the message to the messages array immediately with temporary values
            //todo:  add user id to temp message
            messages.push({ ...chatMessage, uuid: 'temp', message_sent_at: new Date().toISOString() });
            ws.value.send(JSON.stringify(chatMessage));
        }
    }


    return {
        messages,
        connect,
        disconnect,
        sendMessage,
    }
})
