import { createI18n } from 'vue-i18n'

// Import translation files
import en from './translations/en.json'
import de from './translations/de.json'
import LanguageConstant from './stores/models/languageConstant'

const messages = {
    en,
    de,
}

// Get the browser's default language
let userLocale = localStorage.getItem('locale') || navigator.language.split('-')[0] || 'en'
if (!Object.values(LanguageConstant).includes(userLocale)) {
    userLocale = 'en'
}

const i18n = createI18n({
    locale: userLocale,
    fallbackLocale: 'en',
    messages,
})

export default i18n
