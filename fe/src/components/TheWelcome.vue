<script setup>
    import WelcomeItem from './WelcomeItem.vue'
    import DocumentationIcon from './icons/IconDocumentation.vue'
    import ToolingIcon from './icons/IconTooling.vue'
    import EcosystemIcon from './icons/IconEcosystem.vue'
    import CommunityIcon from './icons/IconCommunity.vue'
    import SupportIcon from './icons/IconSupport.vue'
    import { useI18n } from 'vue-i18n'

    const { t } = useI18n()
</script>

<template>
    <WelcomeItem>
        <template #icon>
            <DocumentationIcon />
        </template>
        <template #heading>{{ t('content.welcome.skumb.title') }}</template>

        <p>
            {{ t('content.welcome.skumb.p1') }}
        </p>
        <p>
            {{ t('content.welcome.skumb.p2') }}
        </p>
        <p>
            {{ t('content.welcome.skumb.p3') }}
        </p>
        <p>
            {{ t('content.welcome.skumb.p4') }}
        </p>
    </WelcomeItem>

    <WelcomeItem>
        <template #icon>
            <ToolingIcon />
        </template>
        <template #heading>{{ t('content.welcome.anonymous-chat.title') }}</template>

        <p>
            {{ t('content.welcome.anonymous-chat.p1') }}
            <a
                href="https://skumb.eu"
                title="skumb chat"
            >
                Skumb
            </a>
            .
            <br />
            {{ t('content.welcome.anonymous-chat.p2') }}
        </p>
    </WelcomeItem>

    <WelcomeItem>
        <template #icon>
            <EcosystemIcon />
        </template>
        <template #heading>{{ t('content.welcome.whistle-blowing.title') }}</template>

        <p>
            {{ t('content.welcome.whistle-blowing.p1') }}
            <br />
            {{ t('content.welcome.whistle-blowing.p2') }}
        </p>
    </WelcomeItem>

    <WelcomeItem>
        <template #icon>
            <CommunityIcon />
        </template>
        <template #heading><span v-html="t('content.welcome.surveillance.title')"></span></template>

        <p>
            {{ t('content.welcome.surveillance.p1') }}
            <br />
            {{ t('content.welcome.surveillance.p2') }}
            <br />
            {{ t('content.welcome.surveillance.p3') }}
            <a
                href="https://skumb.eu"
                title="skumb chat"
            >
                {{ t('content.welcome.surveillance.p4') }}
            </a>
            .
        </p>
    </WelcomeItem>

    <WelcomeItem>
        <template #icon>
            <SupportIcon />
        </template>
        <template #heading>{{ t('content.welcome.support.title') }}</template>

        {{ t('content.welcome.support.p1') }}
        <a
            href="mailto:shared-love@skumb.eu"
            target="_blank"
            rel="noopener"
        >
            {{ t('content.welcome.support.p2') }} </a
        >.
    </WelcomeItem>
</template>
