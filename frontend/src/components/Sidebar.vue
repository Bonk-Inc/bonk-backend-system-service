<script setup lang="ts">
import { Joystick, List, Plus } from 'lucide-vue-next';
import { Button } from './ui/button';
import { useRoute, useRouter } from 'vue-router';
import { inject, ref, watch } from 'vue';
import { type GameDTO, type Game } from '@/lib/Models';
import type { ApiService } from '@/lib/ApiService';
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogTitle, DialogTrigger } from './ui/dialog';
import { Label } from './ui/label';
import { Input } from './ui/input';

const route = useRoute();
const router = useRouter();
const apiService = inject<ApiService>('api');

const gameId = ref<string>(route.params.gameId as string ?? '');
const newGame = ref<GameDTO>({ name: '' });
const games = ref<Game[]>([]);
const errors = ref<string>('');

try {
  const gamesResponse = await apiService?.get<Game[]>('api/game/');

  if (gamesResponse)
    games.value = gamesResponse.data;
} catch (e: any) {
  errors.value = e.message;
}

watch(
  () => route.params.gameId,
  (newId) => gameId.value = newId as string ?? ''
)

const onClick = (gameId: string) => {
  router.push(`/app/game/${gameId}`)
};

const saveGame = async () => {
  const body = JSON.stringify(newGame.value);

  try {
    const response = await apiService?.post<Game>('api/game/', body);
    games.value.push(response?.data as Game);
  } catch (e: any) {
    errors.value = e.message;
  }
}

</script>

<template>
  <aside class="w-60 border-border border-r">
    <div class="h-14 flex items-center justify-between px-6">
      <p class="font-bold text-sm flex items-center">
        <List class="mr-2" />
        Games
      </p>
      
      <Dialog>
        <DialogTrigger as-child>
          <Button variant="ghost" size="icon">
            <Plus />
          </Button>
        </DialogTrigger>
        <DialogContent class="sm:max-w-[425px]">
          <DialogTitle>Game toevoegen</DialogTitle>
          <DialogDescription>
            Voeg een nieuwe game toe aan de BABS
          </DialogDescription>

          <form class="grid gap-4 py-4">
            <div class="grid grid-cols-4 items-center gap-4">
              <Label for="game-name">
                Naam*
              </Label>
              <Input 
                @input="event => newGame = { name: event.target.value }" 
                id="game-name" 
                class="col-span-4"
              />
            </div>
          </form>
         
          <DialogFooter>
            <Button @click="saveGame">
              Toevoegen
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
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