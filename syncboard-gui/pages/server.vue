<template>
    <div>
        <banner />
        
        <div class="flex">
            <div role="alert" class="alert bg-primary opacity-75 mx-4 flex">
                <icon name="triangle-exclamation" size="25x25" />
                <span class="text-white">Remember to allow SyncBoard server through your firewall.</span>
            </div>
        </div>

        <div class="flex mt-4">
            <div role="alert" class="alert alert-info opacity-75 mx-4 flex">
                <icon name="circle-info" size="25x25" />
                <span class="text-white">To stop the service, close the SyncBoard application.</span>
            </div>
        </div>

        <div class="flex mt-4">
            <div role="alert" class="alert bg-success opacity-75 mx-4 flex">
                <icon name="check" size="25x25" />
                <span class="text-white">The SyncBoard server service is running.</span>
            </div>
        </div>

        <div class="flex mt-4">
            <div role="alert" class="alert bg-success opacity-75 mx-4 flex">
                <icon name="plug" size="25x25" />
                <span class="text-white">SyncBoard server is using port 9055.</span>
            </div>
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
    async mounted() {
        this.run = await invoke("start_syncboard_server");
    }
}
</script>