<script setup lang="ts">
import { Joystick, List, Plus } from 'lucide-vue-next';
import { Button } from './ui/button';
import { useRoute, useRouter } from 'vue-router';
import { inject, onMounted, ref, watch } from 'vue';
import type { Game } from '@/lib/Models';
import type { ApiService } from '@/lib/ApiService';

const route = useRoute();
const router = useRouter();
const gameId = ref<string>(route.params.gameId as string ?? '');
const apiService = inject<ApiService>('api');

let games: Game[] = [];
let errors: string = '';

try {
  const gamesResponse = await apiService?.get<Game[]>('api/game/');

  if (gamesResponse)
    games = gamesResponse.data;
} catch (e: any) {
  errors = e.message;
}

watch(
  () => route.params.gameId,
  (newId) => gameId.value = newId as string ?? ''
)

const onClick = (gameId: string) => {
  router.push(`/app/game/${gameId}`)
};
</script>

<template>
  <aside class="w-60 border-border border-r">
    <div class="h-14 flex items-center justify-between px-6">
      <p class="font-bold text-sm flex items-center">
        <List class="mr-2" />
        Games
      </p>
      <Button variant="ghost" size="icon">
        <Plus />
      </Button>
    </div>
    <hr class="w-11/12 mx-auto">
    <ul class="py-2 game-menu">
      <li v-for="game in games" class="flex justify-start items-center" :class="{ active: gameId === game.id }">
        <Button variant="ghost" class="w-full rounded-none justify-start px-4 py-2" @click="onClick(game.id)">
          <Joystick class="mr-2" /> {{ game.name }}
        </Button>
      </li>
    </ul>
  </aside>
</template>

<style lang="postcss">
.game-menu .active button {
  @apply shadow-inner-l-solid shadow-blue-600;
}
</style>