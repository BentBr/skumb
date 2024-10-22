<script>
    import { computed, ref } from 'vue'
    import { useChatStore } from '../../stores/chat'
    import Snackbar from '../utils/Snackbar.vue'
    import { useI18n } from 'vue-i18n'

    export default {
        components: { Snackbar },
        setup() {
            const { t } = useI18n()
            const snackbarMessage = computed(() => t('chat.message.copy-uri'))
            const snackbarFailMessage = computed(() => t('chat.message.copy-uri-fail'))
            const chatActiveMessage = computed(() => t('chat.message.active'))
            const chatInactiveMessage = computed(() => t('chat.message.inactive'))

            const snackbarCopyHintRef = ref(null)
            const chatStore = useChatStore()

            const copyUri = () => {
                const uri = chatStore.getChatPath()

                // Modern approach
                if (navigator && navigator.clipboard && navigator.clipboard.writeText) {
                    navigator.clipboard
                        .writeText(uri)
                        .then(() => {
                            console.log(snackbarMessage.value + ': ', uri)
                            snackbarCopyHintRef.value.showSnackbar(snackbarMessage.value)
                        })
                        .catch((err) => {
                            console.error(snackbarFailMessage, err)
                        })
                } else {
                    // Fallback method (old browsers / JS)
                    const textArea = document.createElement('textarea')
                    textArea.value = uri
                    document.body.appendChild(textArea)
                    textArea.select()
                    try {
                        document.execCommand('copy')
                        console.log(snackbarMessage.value + ': ', uri)
                        snackbarCopyHintRef.value.showSnackbar(snackbarMessage.value)
                    } catch (err) {
                        console.error(snackbarFailMessage, err)
                    }
                    document.body.removeChild(textArea)
                }
            }

            return {
                copyUri,
                snackbarCopyHintRef,
                chatStore,
                chatActiveMessage,
                chatInactiveMessage,
            }
        },
    }
</script>

<template>
    <Snackbar
        ref="snackbarCopyHintRef"
        style="padding: 0"
    />

    <p>
        <span v-if="!chatStore.isActive">
            <v-icon
                color="red"
                class="sm-2"
            >
                mdi-circle-small
            </v-icon>
            <small>
                {{ chatInactiveMessage }}
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
            <small>{{ chatActiveMessage }}</small>
        </span>
    </p>
</template>

<style scoped>
    p {
        line-height: 2rem;
    }
</style>
