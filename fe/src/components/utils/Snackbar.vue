<script>
    import { ref } from 'vue'

    export default {
        setup() {
            const snackbar = ref(false)
            const snackbarMessage = ref('')

            const showSnackbar = (message) => {
                snackbarMessage.value = message
                snackbar.value = true
            }

            return {
                snackbar,
                snackbarMessage,
                showSnackbar,
            }
        },
    }
</script>

<template>
    <v-container>
        <v-snackbar
            v-model="snackbar"
            :timeout="3000"
            top
            right
        >
            {{ snackbarMessage }}
        </v-snackbar>
    </v-container>
</template>

<style scoped></style>
