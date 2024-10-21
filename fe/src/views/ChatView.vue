<script>
    import { onMounted, onUnmounted, ref } from 'vue'
    import { useRoute, onBeforeRouteLeave } from 'vue-router'
    import { useChatStore } from '../stores/chat'
    import ChatInput from '../components/ChatInput.vue'
    import ChatOutput from '../components/ChatOutput.vue'
    import ChatHeader from '../components/ChatHeader.vue'

    export default {
        components: { ChatHeader, ChatOutput, ChatInput },
        setup() {
            const route = useRoute()
            const chat_uuid = ref(route.params.chat_uuid) // Access the uuid parameter from the route

            const chatStore = useChatStore()
            const text = ref('')
            const message_sent_at = ref('')
            const usernameEntered = ref(false)
            const chatWindow = ref(null)

            onMounted(() => {
                if (chat_uuid.value) {
                    chatStore.chat_uuid = chat_uuid.value
                }
                if (!chatStore.chat_uuid) {
                    chatStore.fetchChatUuid()
                }
            })

            onUnmounted(() => {
                disconnect()
            })

            onBeforeRouteLeave((to, from, next) => {
                if (from.name === 'shared chat') {
                    chatStore.disconnect()
                }

                next()
            })

            const disconnect = () => {
                chatStore.disconnect()
            }

            return {
                usernameEntered,
                chatStore,
                message_sent_at,
                text,
                chatWindow,
            }
        },
    }
</script>

<template>
    <v-row
        class="d-flex flex-column"
        style="height: calc(100vh - 60px)"
    >
        <v-col
            cols="12"
            class="mx-auto d-flex flex-column"
            style="overflow: hidden"
        >
            <chat-header />
            <div
                class="chat-container flex-grow-1 d-flex flex-column"
                style="overflow-y: auto"
            >
                <chat-output class="flex-grow-1" />
            </div>
            <chat-input />
        </v-col>
    </v-row>
</template>

<style></style>
