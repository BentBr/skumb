import { defineStore } from 'pinia'
import { useI18n } from 'vue-i18n'
import LanguageConstant from './models/languageConstant'

export const useAppStore = defineStore('app', () => {
    const locale = useI18n()

    const changeLanguage = (lang) => {
        if (!Object.values(LanguageConstant).includes(lang)) {
            console.error(`Language ${lang} is not supported)`)

            return
        }

        locale.locale.value = lang
        localStorage.setItem('locale', lang)
    }

    return {
        changeLanguage,
        locale,
    }
})
