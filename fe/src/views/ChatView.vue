<script setup>
    import Chat from '../components/Chat.vue'
</script>

<template>
    <div class="chat">
        <h1>Chat</h1>

        <!-- Suspense component to show to do items  -->
        <Suspense>
            <template #fallback>
                <div>loading</div>
            </template>

            <Chat />
        </Suspense>
    </div>
</template>

<style scoped></style>
