<script setup lang="ts">
import { Joystick, List, Plus } from 'lucide-vue-next';
import { defineProps } from 'vue';
import { Button } from './ui/button';
import { useRoute } from 'vue-router';
import type { Game } from '@/lib/Models';

interface Props {
  games: Game[]
}

const props = defineProps<Props>();
const router = useRoute();
const gameId = router.params.gameId ?? '';
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
      <li v-for="game in props.games" class="flex justify-start items-center" :class="{ active: gameId === game.id }">
        <Button variant="ghost" class="w-full rounded-none justify-start px-4 py-2">
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