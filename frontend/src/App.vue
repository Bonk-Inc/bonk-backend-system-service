<script setup lang="ts">
import { useColorMode } from '@vueuse/core';
import { provide } from 'vue';
import { RouterView } from 'vue-router';

import AuthService from './lib/AuthService';

const authService = new AuthService();
provide('auth', authService);

useColorMode();
</script>

<template>
  <RouterView />
</template>

<style scoped>

</style>
