<template>
  <NuxtLayout>
      <NuxtPage class="p-8" />
  </NuxtLayout>
</template>

<style>
:root {
  --primary: #ffa300;
}
.center {
  margin-left: auto;
  margin-right: auto;
}
</style>

<script setup>
useHead({
  htmlAttrs: {
    'data-theme': 'syncboard'
  }
})
</script>