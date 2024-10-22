<script>
    import { computed, onMounted, onUnmounted, ref } from 'vue'
    import { useRoute, onBeforeRouteLeave } from 'vue-router'
    import { useChatStore } from '../stores/chat'
    import ChatInput from '../components/chat/ChatInput.vue'
    import ChatOutput from '../components/chat/ChatOutput.vue'
    import ChatHeader from '../components/chat/ChatHeader.vue'
    import Routes from '../router/routes'
    import Snackbar from '../components/utils/Snackbar.vue'
    import { useI18n } from 'vue-i18n'

    export default {
        components: { Snackbar, ChatHeader, ChatOutput, ChatInput },
        setup() {
            const { t } = useI18n()
            const snackbarMessage = computed(() => t('chat.message.new-created'))

            const route = useRoute()
            const chat_uuid = ref(route.params.chat_uuid) // Access the uuid parameter from the route
            const chatStore = useChatStore()
            const text = ref('')
            const message_sent_at = ref('')
            const chatWindow = ref(null)
            const snackbarUpdatedChatRef = ref(null)

            onMounted(() => {
                if (chat_uuid.value) {
                    chatStore.chat_uuid = chat_uuid.value
                }
                if (!chatStore.chat_uuid) {
                    chatStore.fetchChatUuid()
                    chatStore.usernameEntered = false
                }
            })

            onUnmounted(() => {
                chatStore.disconnect()
            })

            onBeforeRouteLeave((to, from, next) => {
                if (from.name === Routes.SHARED_CHAT) {
                    chatStore.disconnect()
                }

                if (to.name === Routes.CHAT) {
                    chatStore.fetchChatUuid()
                    snackbarUpdatedChatRef.value.showSnackbar(snackbarMessage)
                    chatStore.usernameEntered = false
                    chatStore.messages = []
                }

                next()
            })

            return {
                chatStore,
                message_sent_at,
                text,
                chatWindow,
                snackbarUpdatedChatRef,
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

    <Snackbar
        ref="snackbarUpdatedChatRef"
        style="padding: 0"
    />
</template>

<style></style>
