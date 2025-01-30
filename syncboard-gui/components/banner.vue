<template>
    <div class="mb-12 text-center">
        <img src="/syncboard.svg" class="w-32 h-32 center" />
        <h1 class="text-primary text-3xl">SyncBoard</h1>
        <p>v{{ version }}</p>
    </div>
</template>

<script>
import { invoke } from "@tauri-apps/api/core";
export default {
    data() {
        return {
            version: ""
        }
    },
    async mounted() {
        this.version = await invoke("get_version");
    }
}
</script>