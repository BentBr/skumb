import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createVuetify } from 'vuetify'
import 'vuetify/styles'
import '@mdi/font/css/materialdesignicons.css'
import {
    VContainer,
    VRow,
    VCol,
    VCard,
    VCardTitle,
    VCardText,
    VBtn,
    VIcon,
    VTextField,
    VExpandTransition,
    VSnackbar,
} from 'vuetify/components'
import { VFadeTransition } from 'vuetify/components/transitions'

import App from './App.vue'
import router from './router'

const vuetify = createVuetify({
    components: {
        VContainer,
        VRow,
        VCol,
        VCard,
        VCardTitle,
        VCardText,
        VBtn,
        VIcon,
        VTextField,
        VExpandTransition,
        VFadeTransition,
        VSnackbar,
    },
})

const app = createApp(App)

app.use(createPinia())
app.use(router)
app.use(vuetify)

app.mount('#app')
