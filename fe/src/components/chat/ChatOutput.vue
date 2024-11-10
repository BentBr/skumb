<script>
    import { nextTick, ref, watch } from 'vue'
    import { useChatStore } from '../../stores/chat'
    import { de } from 'vuetify/locale'

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
        computed: {
            de() {
                return de
            },
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
                :key="msg.user_id + msg.cipher"
                class="chat-message text-grey-darken-4"
            >
                <small>{{ createDateString(msg.message_sent_at) }}</small
                >&nbsp;
                <strong>{{ chatStore.getUserNameForId(msg.user_id) }}:</strong>
                {{ msg.cipher }}
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
