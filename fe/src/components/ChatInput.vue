<script>
    import { computed, nextTick, onMounted, onUnmounted, ref, watch } from 'vue'
    import { useRoute, onBeforeRouteLeave } from 'vue-router'
    import { useChatStore } from '../stores/chat'

    export default {
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

            watch(
                () => chatStore.messages,
                () => {
                    scrollToBottom()
                },
                { deep: true },
            )

            const onUsernameBlur = () => {
                if (chatStore.user_id.trim() !== '') {
                    usernameEntered.value = true
                }

                chatStore.connect()
            }

            const onUsernameEnter = () => {
                if (chatStore.user_id.trim() !== '') {
                    usernameEntered.value = true

                    chatStore.connect()
                }
            }

            const disconnect = () => {
                chatStore.disconnect()
            }

            const send = () => {
                if (text.value.trim() === '' || chatStore.user_id.trim() === '') {
                    return
                }
                chatStore.sendMessage(chatStore.user_id, text.value)

                text.value = ''
                if (!usernameEntered.value) {
                    usernameEntered.value = true
                }
            }

            const scrollToBottom = () => {
                nextTick(() => {
                    if (chatWindow.value) {
                        chatWindow.value.scrollTop = chatWindow.value.scrollHeight
                    }
                })
            }

            const isMobile = computed(() => window.innerWidth <= 768)

            return {
                usernameEntered,
                chatStore,
                message_sent_at,
                text,
                chatWindow,
                isMobile,
                onUsernameBlur,
                onUsernameEnter,
                send,
            }
        },
    }
</script>

<template>
    <div
        ref="chatInput"
        class="chat-input"
    >
        <v-row align="center">
            <v-col>
                <v-expand-transition>
                    <v-text-field
                        v-if="!usernameEntered"
                        v-model="chatStore.user_id"
                        label="Enter your username first"
                        class="mt-4"
                        variant="underlined"
                        @blur="onUsernameBlur"
                        @keyup.enter="onUsernameEnter"
                    ></v-text-field>
                </v-expand-transition>
            </v-col>
        </v-row>

        <div class="d-flex align-center">
            <v-row align="center">
                <v-col>
                    <v-text-field
                        v-model="text"
                        label="Message"
                        class="flex-grow-1"
                        :disabled="!chatStore.isActive"
                        variant="underlined"
                        @keyup.enter="send"
                    ></v-text-field>
                </v-col>
                <v-col cols="auto">
                    <v-btn
                        icon=""
                        class="bg-grey-lighten-1"
                        :disabled="!chatStore.isActive"
                        @click="send"
                    >
                        <v-icon>mdi-send</v-icon>
                    </v-btn>
                </v-col>
            </v-row>
        </div>
    </div>
</template>

<style scoped></style>
