<script setup lang="ts">
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import type { AuthService } from '@/lib/AuthService';
import { AlertCircle, DatabaseZap, KeySquare } from 'lucide-vue-next';
import { inject, onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';

const auth = inject<AuthService>('auth');
const router = useRouter();
const error = ref(false);

onMounted(async () => {
  const loggedIn = await auth?.isUserLoggedIn();
  if (loggedIn) {
    router.push('app_home');
  }
});

const login = async () => {
  try {
    await auth?.login();
  } catch(e) {
    error.value = true;
  }
};
</script>

<template>
  <main class="h-screen grid">
    <Alert v-if="error" variant="destructive" class="absolute top-28 inset-x-0 w-full max-w-xl mx-auto">
      <AlertCircle class="w-4 h-4" />
      <AlertTitle>Fout tijdens het inloggen</AlertTitle>
      <AlertDescription>
        Er is wat fout gegaan tijdens het inloggen, probeer het later opnieuw
      </AlertDescription>
    </Alert>

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
        <Button @click="login">
          <KeySquare color="#fd4b2d" class="mr-2" />
          Inloggen met Authentik
        </Button>
      </CardContent>
    </Card>
  </main>
</template>
