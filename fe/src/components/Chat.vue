<template>
    <v-container>
        <v-row>
            <v-col>
                <v-card class="pa-5">
                    <v-card-title>
                        <p>
                            <span v-if="!isActive">
                                <Snackbar ref="snackbarRef" />
                                <v-icon
                                    color="red"
                                    class="sm-2"
                                >
                                    mdi-circle-small
                                </v-icon>
                                <small>
                                    Your chat is inactive share it!
                                    <v-icon @click="copyUri">mdi-share-variant</v-icon></small
                                >
                            </span>
                            <span v-else>
                                <v-icon
                                    color="green"
                                    class="sm-2"
                                >
                                    mdi-circle-small
                                </v-icon>
                            </span>
                        </p>
                    </v-card-title>
                    <v-card-text>
                        <div class="chat-window" ref="chatWindow">
                            <transition-group
                                name="fade"
                                tag="div"
                            >
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
                            <v-btn
                                icon=""
                                color="primary"
                                :disabled="!isActive"
                                @click="send"
                            >
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
import {computed, nextTick, onMounted, onUnmounted, ref, watch} from 'vue'
    import { useRoute, onBeforeRouteLeave } from 'vue-router'
    import { useChatStore } from '../stores/chat'
    import ConnectionStatus from '../stores/models/connectionStatus'
    import Snackbar from './Snackbar.vue'

    export default {
        components: {
            Snackbar,
        },

        setup() {
            const route = useRoute()
            const chat_uuid = ref(route.params.chat_uuid) // Access the uuid parameter from the route

            const chatStore = useChatStore()
            const text = ref('')
            const message_sent_at = ref('')
            const usernameEntered = ref(false)
            const snackbarRef = ref(null)
            const chatWindow = ref(null);

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

            watch(() => chatStore.messages, () => {
                scrollToBottom();
            }, { deep: true });

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

            const createDateString = (dateString) => {
                const date = new Date(dateString)
                const options = { hour: '2-digit', minute: '2-digit' }

                return new Intl.DateTimeFormat('default', options).format(date)
            }

            const isActive = computed(() => {
                return chatStore.connectionStatus === ConnectionStatus.ACTIVE
            })

            const copyUri = () => {
                const uri = chatStore.getChatPath()

                // Modern approach
                if (navigator && navigator.clipboard && navigator.clipboard.writeText) {
                    navigator.clipboard
                        .writeText(uri)
                        .then(() => {
                            console.log('URI copied to clipboard:', uri)
                            snackbarRef.value.showSnackbar('URI copied to clipboard')
                        })
                        .catch((err) => {
                            console.error('Failed to copy URI:', err)
                        })
                } else {
                    // Fallback method
                    const textArea = document.createElement('textarea')
                    textArea.value = uri
                    document.body.appendChild(textArea)
                    textArea.select()
                    try {
                        document.execCommand('copy')
                        console.log('URI copied to clipboard:', uri)
                        snackbarRef.value.showSnackbar('URI copied to clipboard')
                    } catch (err) {
                        console.error('Failed to copy URI:', err)
                    }
                    document.body.removeChild(textArea)
                }
            }

            const scrollToBottom = () => {
                nextTick(() => {
                    if (chatWindow.value) {
                        chatWindow.value.scrollTop = chatWindow.value.scrollHeight;
                    }
                });
            };

            return {
                usernameEntered,
                chatStore,
                message_sent_at,
                text,
                isActive,
                snackbarRef,
                chatWindow,
                onUsernameBlur,
                onUsernameEnter,
                send,
                createDateString,
                copyUri,
            }
        },
    }
</script>

<style>
    .chat-window {
        height: 400px;
        overflow-y: auto;
        background-color: #f5f5f5;
        padding: 15px 15px 0 15px;
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
