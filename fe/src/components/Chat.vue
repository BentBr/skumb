<script>
    import {onMounted, onUnmounted, ref} from 'vue';
    import {useRoute} from 'vue-router';
    import { useChatStore } from '../stores/chat';

    export default {
        setup() {
            const route = useRoute();
            const chat_uuid = ref(route.params.chat_uuid); // Access the uuid parameter from the route

            const chatStore = useChatStore();
            const user_uuid = ref('');
            const text = ref('');
            const message_sent_at = ref('');
            const uuid = ref('');

            onMounted(() => {
                chatStore.connect(chat_uuid.value);
            });

            // todo: add disconnect for leaving page or clicking out or setting browser tab to background
            // onUnmounted(() => {
            //     chatStore.disconnect();
            // });

            const send = () => {
                chatStore.sendMessage(user_uuid.value, text.value);
                text.value = '';
            };

            const createDateString = (dateString) => {
                const date = new Date(dateString);
                const options = { hour: '2-digit', minute: '2-digit' };

                return new Intl.DateTimeFormat('default', options).format(date);
            };

            return {
                chatStore,
                user_uuid,
                message_sent_at,
                uuid,
                text,
                send,
                createDateString
            };
        },
    };
</script>

<!--<template>-->
<!--    <div>-->
<!--        <ChatOutput/>-->
<!--        <ChatInput/>-->


<!--    </div>-->
<!--</template>-->



<template>
    <div>
        <h1>Chat</h1>
        <div v-for="msg in chatStore.messages" :key="msg.user_uuid + msg.text">
            <small>{{ createDateString(msg.message_sent_at) }}</small> <strong>{{ msg.user_uuid }}:</strong> {{ msg.text }}
        </div>
        <input v-model="user_uuid" placeholder="Username" />
        <input v-model="text" placeholder="Message" @keyup.enter="send" />
        <button @click="send">Send</button>
    </div>
</template>

<style>
</style>
