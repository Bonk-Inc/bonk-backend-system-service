<script setup lang="ts">
import GameLayout from '@/components/layout/GameLayout.vue';
import { Card, CardContent } from '@/components/ui/card';
import { useToast } from '@/components/ui/toast';
import { ApiService } from '@/lib/ApiService';
import type { GameStats } from '@/lib/Models';
import { Tally5, Users } from 'lucide-vue-next';
import { inject, ref, watch } from 'vue';
import { useRoute } from 'vue-router';

const route = useRoute();
const apiService = inject<ApiService>('api');
const { toast } = useToast();

const stats = ref<GameStats>();

watch(
  () => route.params.gameId,
  async (newId) => await fetchStats(newId as string),
  { immediate: true }
);

async function fetchStats(gameId: string) {
  try {
    const statsResponse = await apiService?.get<GameStats>(`api/stats/game/${gameId}`);
    stats.value = statsResponse?.data;
  } catch(e: unknown) {
    toast({
      title: 'Er ging wat fout :(',
      variant: 'destructive',
      description: 'Er is wat fout gegaan tijdens het ophalen van de game statistieken'
    });
  }
}
</script>

<template>
  <GameLayout>
    <section class="mt-4">
      <h2 class="text-xl font-medium">Statistieken</h2>
      <div class="grid gap-5 grid-cols-5 w-full mt-4 ">
        <Card class="w-[288px]">
          <CardContent class="pt-6 flex justify-between items-center">
            <Tally5 :size="36" class="mr-2 text-blue-500" />
            <div>
              <p class="mb-3">Scores</p>
              <p class="mt-3 text-xl text-right text-blue-500">{{ stats?.scores }}</p>
            </div>
          </CardContent>
        </Card>
        <Card class="w-[288px]">
          <CardContent class="pt-6 flex justify-between items-center">
            <Users :size="36" class="mr-2 text-blue-500" />
            <div>
              <p class="mb-3">Gebruikers</p>
              <p class="mt-3 text-xl text-right text-blue-500">{{ stats?.users }}</p>
            </div>
          </CardContent>
        </Card>
      </div> 
    </section>
  </GameLayout>
</template>