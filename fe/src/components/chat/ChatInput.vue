<script>
    import { computed, nextTick, ref, watch } from 'vue'
    import { useChatStore } from '../../stores/chat'
    import { useI18n } from 'vue-i18n'

    export default {
        setup() {
            const { t } = useI18n()
            const usernameLabel = computed(() => t('chat.label.username'))
            const messageLabel = computed(() => t('chat.label.message'))

            const chatStore = useChatStore()
            const text = ref('')
            const message_sent_at = ref('')
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
                    chatStore.usernameEntered = true
                }

                chatStore.connect()
            }

            const onUsernameEnter = () => {
                if (chatStore.user_id.trim() !== '') {
                    chatStore.usernameEntered = true

                    chatStore.connect()
                }
            }

            const send = () => {
                if (text.value.trim() === '' || chatStore.user_id.trim() === '') {
                    return
                }
                chatStore.sendMessage(chatStore.user_id, text.value)

                text.value = ''
                if (!chatStore.usernameEntered) {
                    chatStore.usernameEntered = true
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
                chatStore,
                message_sent_at,
                text,
                chatWindow,
                usernameLabel,
                messageLabel,
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
                        v-if="!chatStore.usernameEntered"
                        v-model="chatStore.user_id"
                        :label="usernameLabel"
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
                        :label="messageLabel"
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
