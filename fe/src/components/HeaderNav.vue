<script>
    import { defineComponent, onMounted, ref, watch } from 'vue'
    import { RouterLink, useRoute } from 'vue-router'

    export default defineComponent({
        components: { RouterLink },
        setup() {
            const route = useRoute()
            const selectedTab = ref('')

            const updateSelectedTab = () => {
                selectedTab.value = route.name
            }

            onMounted(updateSelectedTab)

            watch(route, updateSelectedTab)

            return {
                selectedTab,
            }
        },
    })
</script>

<template>
    <v-app-bar
        class="px-3"
        density="compact"
        flat
    >
        <v-avatar
            class="hidden-md-and-up"
            color="grey-darken-1"
            size="32"
        ></v-avatar>

        <v-spacer></v-spacer>

        <nav>
            <v-tabs
                v-model="selectedTab"
                color="grey-darken-2"
                centered
            >
                <v-tab value="home">
                    <RouterLink to="/">Home</RouterLink>
                </v-tab>
                <v-tab value="shared-chat">
                    <RouterLink to="/chat">Chat</RouterLink>
                </v-tab>
            </v-tabs>
        </nav>
        <v-spacer></v-spacer>

        <v-avatar
            class="hidden-sm-and-down"
            color="grey-darken-1"
            size="32"
        ></v-avatar>
    </v-app-bar>
</template>

<style scoped></style>
