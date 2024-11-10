<script setup>
    import WelcomeItem from './WelcomeItem.vue'
    import DocumentationIcon from './icons/IconDocumentation.vue'
    import ToolingIcon from './icons/IconTooling.vue'
    import EcosystemIcon from './icons/IconEcosystem.vue'
    import CommunityIcon from './icons/IconCommunity.vue'
    import SupportIcon from './icons/IconSupport.vue'
</script>

<template>
    <WelcomeItem>
        <template #icon>
            <DocumentationIcon />
        </template>
        <template #heading>Skumb</template>

        <p>
            Skumb is the idea of a simple chat which is anonymous, secure and volatile.
        </p>
        <p>
            You can simply create a new chat without any registration and share the link with your friends, colleagues or family.
        </p>
        <p>
            Every chat is end-to-end encrypted with latest and proven as secure algorithms.
        </p>
        <p>
            The chat is volatile, which means that the chat is not stored anywhere and leaving the page or refreshing it will delete all the messages (current anonymous version).
        </p>
    </WelcomeItem>

    <WelcomeItem>
        <template #icon>
            <ToolingIcon />
        </template>
        <template #heading>Anonymous chat</template>

        <p>
            You want to quickly share a private key? Some password? Or other sensitive information which you don't want to store anywhere? Use
            <a
                href="https://skumb.eu"
                title="skumb chat"
            >
                Skumb
            </a>
            .
            <br />
            No questions asked, no data stored, no registration needed. No possibility to recover the chat, your data, your messages.
        </p>
    </WelcomeItem>

    <WelcomeItem>
        <template #icon>
            <EcosystemIcon />
        </template>
        <template #heading>Whistleblowing (coming soon)</template>

        <p>
            You are a company or a collective of journalists and you want a way to be connected securely and anonymously?
            <br />
            With Skumb you can provide an option to be contacted in anonymity and security.
            Get information about breaches, leaks or other topics of need first-hand and have those properly secure stored and only accessible by you.
        </p>
    </WelcomeItem>

    <WelcomeItem>
        <template #icon>
            <CommunityIcon />
        </template>
        <template #heading>Chat without remorse (coming soon)</template>

        <p>
            You want to communicate in a modern and secure way without the scratching itch of some company evaluating your messages or meta data?
            <br />
            Don't support blind data collection and surveillance. Use Skumb.
            <br />
            Don't support questionable data retention policies. Use Skumb.
            <br />
            Don't support the data economy. Use Skumb.
            <br />
            Don't support questionable oligarchies. Use Skumb.
            <br />
            Support privacy, security, anonymity and a shared love for human rights, dignity and a proper freedom of speech.
            <a
                href="https://skumb.eu"
                title="skumb chat"
            >
                Use Skumb
            </a>
            .
        </p>
    </WelcomeItem>

    <WelcomeItem>
        <template #icon>
            <SupportIcon />
        </template>
        <template #heading>Made with love in Rust and Vue (support coming soon)</template>

        As an independent project, we are always open for support and some shared love.
        <a
            href="mailto:shared-love@skumb.eu"
            target="_blank"
            rel="noopener"
        >
            Become a supporter
        </a>.
    </WelcomeItem>
</template>
