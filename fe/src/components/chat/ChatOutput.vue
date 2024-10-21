<script>
    import { nextTick, ref, watch } from 'vue'
    import { useChatStore } from '../../stores/chat'

    export default {
        setup() {
            const chatStore = useChatStore()
            const text = ref('')
            const message_sent_at = ref('')
            const usernameEntered = ref(false)
            const chatWindow = ref(null)

            watch(
                () => chatStore.messages,
                () => {
                    scrollToBottom()
                },
                { deep: true },
            )

            const createDateString = (dateString) => {
                const date = new Date(dateString)
                const options = { hour: '2-digit', minute: '2-digit' }

                return new Intl.DateTimeFormat('default', options).format(date)
            }

            const scrollToBottom = () => {
                nextTick(() => {
                    if (chatWindow.value) {
                        console.log(chatWindow.value.scrollTop)
                        chatWindow.value.scrollTop = chatWindow.value.scrollHeight
                    }
                })
            }

            return {
                usernameEntered,
                chatStore,
                message_sent_at,
                text,
                chatWindow,
                createDateString,
            }
        },
    }
</script>

<template>
    <div
        ref="chatWindow"
        class="chat-output flex-grow-1"
    >
        <transition-group
            name="fade"
            tag="div"
        >
            <div
                v-for="msg in chatStore.messages"
                :key="msg.user_id + msg.text"
                class="chat-message text-grey-darken-4"
            >
                <small>{{ createDateString(msg.message_sent_at) }}</small
                >&nbsp;
                <strong>{{ msg.user_id }}:</strong>
                {{ msg.text }}
            </div>
        </transition-group>
    </div>
</template>

<style scoped>
    .chat-output {
        overflow-y: auto;
        background-color: #f5f5f5;
        padding: 15px 15px 0 15px;
        border-radius: 8px;
        flex-grow: 1;
    }

    .chat-message {
        margin-bottom: 10px;
        word-break: break-word; /* Add this line to break long words */
    }
</style>
