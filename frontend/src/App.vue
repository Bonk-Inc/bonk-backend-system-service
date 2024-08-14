<script setup lang="ts">
import { useColorMode } from '@vueuse/core';
import { RouterView } from 'vue-router';

useColorMode();
</script>

<template>
  <RouterView />
</template>

<style scoped>

</style>
