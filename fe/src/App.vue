<script>
    import HeaderNav from './components/HeaderNav.vue'

    export default {
        components: { HeaderNav },
    }
</script>

<template>
    <v-app>
        <HeaderNav />

        <v-main class="bg-grey-darken-4">
            <v-container>
                <RouterView />
            </v-container>
        </v-main>
    </v-app>
</template>

<style scoped></style>
