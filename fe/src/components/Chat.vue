<template>
    <v-container>
        <v-row>
            <v-col>
                <v-card class="pa-5">
                    <v-card-title>
                        <p>
                            <span v-if="!isActive">
                                <v-icon color="red" class="sm-2"> mdi-circle-small </v-icon>
                                <small>Your chat is inactive</small>
                            </span>
                            <span v-else>
                                <v-icon color="green" class="sm-2"> mdi-circle-small </v-icon>
                            </span>
                        </p>
                    </v-card-title>
                    <v-card-text>
                        <div class="chat-window">
                            <transition-group name="fade" tag="div">
                                <div
                                    v-for="msg in chatStore.messages"
                                    :key="msg.user_id + msg.text"
                                    class="chat-message"
                                >
                                    <small>{{ createDateString(msg.message_sent_at) }}</small
                                    >&nbsp;
                                    <strong>{{ msg.user_id }}:</strong>
                                    {{ msg.text }}
                                </div>
                            </transition-group>
                        </div>
                        <v-expand-transition>
                            <v-text-field
                                v-if="!usernameEntered"
                                v-model="chatStore.user_id"
                                label="Username"
                                class="mt-4"
                                @blur="onUsernameBlur"
                                @keyup.enter="onUsernameEnter"
                            ></v-text-field>
                        </v-expand-transition>
                        <div class="d-flex align-center">
                            <v-text-field
                                v-model="text"
                                label="Message"
                                class="flex-grow-1"
                                :disabled="!isActive"
                                @keyup.enter="send"
                            ></v-text-field>
                            <v-btn icon="" color="primary" :disabled="!isActive" @click="send">
                                <v-icon>mdi-send</v-icon>
                            </v-btn>
                        </div>
                    </v-card-text>
                </v-card>
            </v-col>
        </v-row>
    </v-container>
</template>

<script>
    import { computed, onMounted, onUnmounted, ref } from 'vue'
    import { useRoute } from 'vue-router'
    import { useChatStore } from '../stores/chat'
    import ConnectionStatus from '../stores/models/connectionStatus'

    export default {
        beforeRouteLeave(to, from, next) {
            this.chatStore.disconnect()
            next()
        },
        setup() {
            const route = useRoute()
            const chat_uuid = ref(route.params.chat_uuid) // Access the uuid parameter from the route

            const chatStore = useChatStore()
            const text = ref('')
            const message_sent_at = ref('')
            const usernameEntered = ref(false)

            onMounted(() => {
                if (chat_uuid.value) {
                    chatStore.chat_uuid = chat_uuid.value
                }
                if (!chatStore.chat_uuid) {
                    chatStore.fetchChatUuid()
                }
            })

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

            onUnmounted(() => {
                disconnect()
            })

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

            const createDateString = (dateString) => {
                const date = new Date(dateString)
                const options = { hour: '2-digit', minute: '2-digit' }

                return new Intl.DateTimeFormat('default', options).format(date)
            }

            const isActive = computed(() => {
                return chatStore.connectionStatus === ConnectionStatus.ACTIVE
            })

            return {
                usernameEntered,
                chatStore,
                message_sent_at,
                text,
                isActive,
                onUsernameBlur,
                onUsernameEnter,
                send,
                createDateString,
            }
        },
    }
</script>

<style>
    .chat-window {
        height: 400px;
        overflow-y: auto;
        background-color: #f5f5f5;
        padding: 15px;
        border-radius: 8px;
    }

    .chat-message {
        margin-bottom: 10px;
    }

    .fade-enter-active,
    .fade-leave-active {
        transition: opacity 0.75s;
    }

    .fade-enter-from,
    .fade-leave-to {
        opacity: 0;
    }

    v-card-title {
        h1 {
            font-size: 0.5rem;
        }
    }
</style>
