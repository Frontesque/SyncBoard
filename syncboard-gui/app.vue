<template>
  <NuxtLayout>
      <NuxtPage></NuxtPage>
  </NuxtLayout>
</template>

<script setup>
useHead({
  htmlAttrs: {
    'data-theme': 'syncboard'
  }
})
</script>