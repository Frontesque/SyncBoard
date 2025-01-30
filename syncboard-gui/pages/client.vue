<template>
    <div>
        <banner />

        <div v-if="client_connected">
            <div class="flex mt-4">
                <div role="alert" class="alert alert-info opacity-75 mx-4 flex">
                    <icon name="circle-info" size="25x25" />
                    <span class="text-white">To stop the service, close the SyncBoard application.</span>
                </div>
            </div>

            <div class="flex mt-4">
                <div role="alert" class="alert bg-success opacity-75 mx-4 flex">
                    <icon name="check" size="25x25" />
                    <span class="text-white">The SyncBoard client service is running.</span>
                </div>
            </div>

            <div class="flex mt-4">
                <div role="alert" class="alert bg-success opacity-75 mx-4 flex">
                    <icon name="plug" size="25x25" />
                    <span class="text-white">SyncBoard is connected to "{{address}}".</span>
                </div>
            </div>
        </div>

        <div v-else>
            <div class="flex">
                <input id="sb_address" type="text" placeholder="Enter a SyncBoard server address" class="input input-bordered w-full" />
                <button class="btn btn-primary ml-4" @click="run">Connect</button>
            </div>
        </div>

    </div>
</template>

<script>
import { invoke } from "@tauri-apps/api/core";
export default {
    data() {
        return {
            client_connected: false,
            address: "",
        }
    },
    methods: {
        async run() {
            const syncboard_address = document.getElementById("sb_address").value;
            const result = await invoke("start_syncboard_client", { address: syncboard_address });
            this.address = syncboard_address;
            this.client_connected = true;
        }
    }
}
</script>