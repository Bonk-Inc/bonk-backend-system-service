<script setup lang="ts">
import MainLayout from '@/components/layout/MainLayout.vue';
import { Card, CardContent } from '@/components/ui/card';
import type { AuthService } from '@/lib/AuthService';
import { Joystick, Tally5 } from 'lucide-vue-next';
import type { UserProfile } from 'oidc-client-ts';
import { inject, onMounted, ref } from 'vue';

const auth = inject<AuthService>('auth');
const user = ref<UserProfile | undefined>();

onMounted(async () => {
  user.value = await auth?.getProfile();
  console.log(await auth?.getProfile());
})

</script>

<template>
  <MainLayout>
    <h1 class="text-3xl font-medium">
      Welkom! {{ user?.name }}
    </h1>
    <section class="mt-8">
      <h2 class="text-xl font-medium">Statistieken</h2>
      <div class="grid gap-5 grid-cols-5 w-full mt-4 ">
        <Card class="w-[288px]">
          <CardContent class="pt-6 flex justify-between items-center">
            <Joystick :size="36" class="mr-2 text-blue-500" />
            <div>
              <p class="mb-3">Games</p>
              <p class="mt-3 text-xl text-right text-blue-500">6</p>
            </div>
          </CardContent>
        </Card>
        <Card class="w-[288px]">
          <CardContent class="pt-6 flex justify-between items-center">
            <Tally5 :size="36" class="mr-2 text-blue-500" />
            <div>
              <p class="mb-3">Scores</p>
              <p class="mt-3 text-xl text-right text-blue-500">6</p>
            </div>
          </CardContent>
        </Card>
      </div>
    </section>
  </MainLayout>
</template>