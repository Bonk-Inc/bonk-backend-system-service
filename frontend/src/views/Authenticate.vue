<script setup lang="ts">
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { DatabaseZap, KeySquare } from 'lucide-vue-next';
</script>

<template>
  <main class="h-screen grid">
    <Card class="w-full max-w-md place-self-center">
      <CardHeader class="text-center">
        <CardTitle class="flex justify-center items-center">
          <DatabaseZap :size="48" class="mr-4" />
          BABS
        </CardTitle>
        <CardDescription class="text-xl mt-4 text-primary">
          Inloggen in Bonk Inc. Backend System
        </CardDescription>
      </CardHeader>
      <CardContent class="grid my-6 gap-2">
        <Button>
          <KeySquare color="#fd4b2d" class="mr-2" />
          Inloggen met Authentik
        </Button>
      </CardContent>
    </Card>
  </main>
</template>
