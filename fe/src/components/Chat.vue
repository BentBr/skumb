<template>
    <v-container>
        <v-row>
            <v-col>
                <v-card class="pa-5">
                    <v-card-title>
                        <h1>Chat</h1>
                    </v-card-title>
                    <v-card-text>
                        <div class="chat-window">
                            <transition-group name="fade" tag="div">
                                <div v-for="msg in chatStore.messages" :key="msg.user_uuid + msg.text" class="chat-message">
                                    <small>{{ createDateString(msg.message_sent_at) }}</small>&nbsp;
                                    <strong>{{ msg.user_uuid }}:</strong>
                                    {{ msg.text }}
                                </div>
                            </transition-group>
                        </div>
                        <v-expand-transition>
                            <v-text-field
                                v-if="!usernameEntered"
                                v-model="user_id"
                                label="Username"
                                class="mt-4"
                                @blur="onUsernameBlur"
                                @keyup.enter="onUsernameEnter"
                            ></v-text-field>
                        </v-expand-transition>
                        <div class="d-flex align-center">
                            <v-text-field v-model="text" label="Message" @keyup.enter="send" class="flex-grow-1"></v-text-field>
                            <v-btn icon="" @click="send" color="primary">
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
    import {onMounted, onUnmounted, ref} from 'vue';
    import {useRoute} from 'vue-router';
    import { useChatStore } from '../stores/chat';

    export default {
        setup() {
            const route = useRoute();
            const chat_uuid = ref(route.params.chat_uuid); // Access the uuid parameter from the route

            const chatStore = useChatStore();
            const user_id = ref('');
            const text = ref('');
            const message_sent_at = ref('');
            const uuid = ref('');
            const usernameEntered = ref(false)

            const onUsernameBlur = () => {
                if (user_id.value.trim() !== '') {
                    usernameEntered.value = true
                }
            }

            const onUsernameEnter = () => {
                if (user_id.value.trim() !== '') {
                    usernameEntered.value = true
                }
            }

            const connect = () => {
                chatStore.connect(chat_uuid.value);
            };

            const disconnect = () => {
                chatStore.disconnect();
            };

            onMounted(() => {
                connect();
            });

            onUnmounted(() => {
                disconnect();
            });

            const send = () => {
                if (text.value.trim() === '' || user_id.value.trim() === '') {
                    return
                }
                chatStore.sendMessage(user_id.value, text.value);

                text.value = ''
                if (!usernameEntered.value) {
                    usernameEntered.value = true
                }
            }

            const createDateString = (dateString) => {
                const date = new Date(dateString);
                const options = { hour: '2-digit', minute: '2-digit' };

                return new Intl.DateTimeFormat('default', options).format(date);
            };

            return {
                usernameEntered,
                chatStore,
                user_id,
                message_sent_at,
                uuid,
                text,
                onUsernameBlur,
                onUsernameEnter,
                send,
                createDateString
            };
        },

        beforeRouteLeave(to, from, next) {
            this.chatStore.disconnect();
            next();
        }
    };
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

    .fade-enter-active, .fade-leave-active {
        transition: opacity 0.75s;
    }

    .fade-enter-from, .fade-leave-to {
        opacity: 0;
    }
</style>
