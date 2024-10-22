<script>
    import { useAppStore } from '../../stores/app'
    import LanguageConstant from '../../stores/models/languageConstant'
    import { useI18n } from 'vue-i18n'
    import { computed } from 'vue'
    import { useDisplay } from 'vuetify'

    export default {
        setup() {
            const appStore = useAppStore()
            const { t } = useI18n()
            const { smAndDown } = useDisplay() // Get breakpoint for small screens

            const languages = computed(() => [
                { title: t('app.translation.english'), value: LanguageConstant.EN },
                { title: t('app.translation.german'), value: LanguageConstant.DE },
            ])
            const languagesShort = computed(() => [
                { title: t('app.translation.short.english'), value: LanguageConstant.EN },
                { title: t('app.translation.short.german'), value: LanguageConstant.DE },
            ])

            const changeLanguage = (lang) => {
                appStore.changeLanguage(lang)
            }

            const languageItems = computed(() => (smAndDown.value ? languagesShort.value : languages.value))
            return {
                changeLanguage,
                LanguageConstant,
                languageItems,
                appStore,
            }
        },
    }
</script>

<template>
    <div class="language-select">
        <v-select
            v-model="appStore.locale.locale"
            :items="languageItems"
            variant="solo"
            dense
            hide-details="auto"
            hide-selected
            flat
            @update:model-value="changeLanguage"
        ></v-select>
    </div>
</template>

<style scoped></style>
