<template>
    <div class="p-8">
        
        <h1 class="text-3xl text-white text-center">Select a mode to run SyncBoard</h1>

        <div class="flex">
            <button class="box">
                <icon name="server" size="50x50" class="center-icon" />
                <p class="text-3xl text-white">Server</p>
            </button>
            <button class="box">
                <icon name="display" size="50x50" class="center-icon" />
                <p class="text-3xl text-white">Client</p>
            </button>
        </div>
        <button class="box flex">
            <icon name="flask-vial" size="40x40" />
            <p class="text-3xl text-white ml-4">Test</p>
        </button>

    </div>
</template>

<style scoped>
.box {
    width: 100%;
    text-align: center;
    justify-content: center;
    justify-items: center;
    align-items: center;
    align-content: center;
    

    border-radius: 1em;
    border: 3px solid var(--primary);
    margin: 1em;
    padding: 1em;
}

.center-icon {
    margin-left: auto;
    margin-right: auto;
}
</style>