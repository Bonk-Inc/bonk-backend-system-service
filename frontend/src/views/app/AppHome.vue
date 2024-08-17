<script setup lang="ts">
import MainLayout from '@/components/layout/MainLayout.vue';
import type { AuthService } from '@/lib/AuthService';
import { inject, onMounted } from 'vue';

const auth = inject<AuthService>('auth');

</script>

<template>
  <MainLayout>
    <h1>Home</h1>
  </MainLayout>
</template>