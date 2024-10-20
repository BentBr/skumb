<script>
    import { ref } from 'vue'
    import { useChatStore } from '../stores/chat'
    import Snackbar from './Snackbar.vue'

    export default {
        components: { Snackbar },
        setup() {
            const snackbarMessage = 'Link copied to clipboard'
            const snackbarFailMessage = 'Failed to copy URI: '
            const snackbarRef = ref(null)
            const chatStore = useChatStore()

            const copyUri = () => {
                const uri = chatStore.getChatPath()

                // Modern approach
                if (navigator && navigator.clipboard && navigator.clipboard.writeText) {
                    navigator.clipboard
                        .writeText(uri)
                        .then(() => {
                            console.log(snackbarMessage + ': ', uri)
                            snackbarRef.value.showSnackbar(snackbarMessage)
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
                        console.log(snackbarMessage + ': ', uri)
                        snackbarRef.value.showSnackbar(snackbarMessage)
                    } catch (err) {
                        console.error(snackbarFailMessage, err)
                    }
                    document.body.removeChild(textArea)
                }
            }

            return {
                copyUri,
                snackbarRef,
                chatStore,
            }
        },
    }
</script>

<template>
    <Snackbar
        ref="snackbarRef"
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
                Your chat is inactive. Share it!
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
            <small> Your chat is active! </small>
        </span>
    </p>
</template>

<style scoped>
    p {
        line-height: 2rem;
    }
</style>
