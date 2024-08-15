<script setup lang="ts">
import type { AuthService } from '@/lib/AuthService';
import { inject, onMounted } from 'vue';

const auth = inject<AuthService>('auth');

</script>

<template>
    <h1>Home</h1>
</template>