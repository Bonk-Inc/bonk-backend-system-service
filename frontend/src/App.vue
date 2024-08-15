<script setup lang="ts">
import { useColorMode } from '@vueuse/core';
import { provide } from 'vue';
import { RouterView } from 'vue-router';

import { authService } from './lib/AuthService';

provide('auth', authService);

useColorMode();
</script>

<template>
  <RouterView />
</template>

<style scoped>

</style>
