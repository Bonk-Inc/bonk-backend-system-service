<script setup lang="ts">
import { useColorMode } from '@vueuse/core';
import { provide } from 'vue';
import { RouterView, useRouter } from 'vue-router';

import { authService } from './lib/AuthService';
import { apiService } from './lib/ApiService';

provide('auth', authService);
provide('api', apiService(useRouter()));

useColorMode();
</script>

<template>
  <RouterView />
</template>

<style scoped>

</style>
