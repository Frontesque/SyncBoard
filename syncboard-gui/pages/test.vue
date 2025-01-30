<template>
    <div>
        <banner />
        
        <div class="flex items-center">
            <NuxtLink to="/" class="btn mr-2">Back</NuxtLink>
            <button class="btn btn-primary" @click="test">Run Tests</button>
        </div>

        <div v-if="old_clip">
            <div class="mt-4">
                <p class="text-bold text-white">Your old clipboard contents:</p>
                <p style="white-space: pre;">{{ old_clip }}</p>
            </div>
            <p class="mt-4 text-bold text-white">Try pasting your new clipboard contents to ensure SyncBoard can set your clipboard.</p>
        </div>

    </div>
</template>

<script>
import { invoke } from "@tauri-apps/api/core";
export default {
    data() {
        return {
            old_clip: false
        }
    },
    methods: {
        async test() {
            this.old_clip = await invoke("syncboard_test");
        }
    }
}
</script>