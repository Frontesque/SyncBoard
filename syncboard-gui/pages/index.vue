<template>
    <div>
        <banner />
        <!--   Mode Selection   -->
        <h2 class="text-3xl text-white text-center">Select a mode</h2>
        <div class="flex">
            <NuxtLink to="/server" class="box">
                <icon name="server" size="50x50" class="center" />
                <p class="text-3xl text-white">Server</p>
            </NuxtLink>
            <NuxtLink to="/client" class="box">
                <icon name="display" size="50x50" class="center" />
                <p class="text-3xl text-white">Client</p>
            </NuxtLink>
        </div>
        <div class="flex">
            <NuxtLink to="/test"  class="box flex">
                <icon name="flask-vial" size="40x40" />
                <p class="text-3xl text-white ml-4">Test</p>
            </NuxtLink>
        </div>

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
</style>