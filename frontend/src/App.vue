<script setup lang="ts">
import { useColorMode } from '@vueuse/core';
import { provide } from 'vue';
import { RouterView } from 'vue-router';

import { authService } from './lib/AuthService';
import { apiService } from './lib/ApiService';

provide('auth', authService);
provide('api', apiService);

useColorMode();
</script>

<template>
  <RouterView />
</template>

<style scoped>

</style>
