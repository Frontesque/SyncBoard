<template>
  <NuxtLayout>
      <NuxtPage></NuxtPage>
  </NuxtLayout>
</template>

<style>
:root {
  --primary: #8867c0;
}
</style>

<script setup>
useHead({
  htmlAttrs: {
    'data-theme': 'syncboard'
  }
})
</script>