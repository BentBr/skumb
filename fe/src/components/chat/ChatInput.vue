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

            return {
                usernameEntered,
                chatStore,
                message_sent_at,
                text,
                chatWindow,
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
