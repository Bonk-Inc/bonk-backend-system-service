<script setup lang="ts">
import GameLayout from '@/components/layout/GameLayout.vue';
import { Button } from '@/components/ui/button';
import { Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogTitle, DialogTrigger } from '@/components/ui/dialog';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { useToast } from '@/components/ui/toast';
import type { ApiService } from '@/lib/ApiService';
import type { Game } from '@/lib/Models';
import { Copy, Save, Trash } from 'lucide-vue-next';
import { inject, onMounted, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';

const route = useRoute();
const router = useRouter();
const apiService = inject<ApiService>('api');
const { toast } = useToast();

const gameId = route.params.gameId as string;
let game = ref<Game>();
let editedGame = ref<Game>();

onMounted(async () => {
  await fetchGame();
});

const fetchGame = async () => {
  try {
    const response = await apiService?.get<Game>(`api/game/${gameId}`);
    game.value = response?.data;
    editedGame.value = response?.data;
  } catch (e: unknown) {
    toast({
      title: 'Er ging wat fout :(',
      variant: 'destructive',
      description: 'Kon geen game informatie ophalen'
    });
  }
}

const copyGameId = async () => {
  try {
    await navigator.clipboard.writeText(gameId);

    toast({
      description: 'Game ID gekopieerd!'
    });
  } catch (e: unknown) {
    console.error(e);
  }
}

const updateGame = async() => {
  try {
    const body = JSON.stringify(editedGame.value);

    await apiService?.put(`api/game/${gameId}`, body);
    await fetchGame();

    toast({
      title: 'Gelukt!',
      description: 'Game is geupdate'
    });
  } catch(e: unknown) {
    toast({
      title: 'Er ging wat fout :(',
      variant: 'destructive',
      description: 'Er is wat fout gegaan tijdens het updaten van de game'
    });
  }
}

const deleteGame = async() => {
  try {
    await apiService?.delete(`api/game/${gameId}`);
    router.push({ name: 'app_home' });
  } catch(e: unknown) {
    toast({
      title: 'Er ging wat fout :(',
      variant: 'destructive',
      description: 'Er is wat fout gegaan tijdens het verwijderen van de game'
    });
  }
}
</script>

<template>
  <GameLayout>
    
    <h2 class="text-xl font-medium">Instellingen</h2>
    <div class="grid grid-cols-4">
      <div class="py-4 col-span-4">
        <p class="font-bold">Game ID:</p>
        <div class="flex items-center">
          <p>{{gameId}}</p>
          <Button variant="ghost" size="icon" class="ml-4">
            <Copy :size="20" @click="copyGameId()" />
          </Button>
        </div>
      </div>
      <div class="py-4">
        <Label for="game-name" class="font-bold text-base">
          Naam:
        </Label>
        <Input
          :model-value="editedGame?.name"
          @input="(event: Event) => editedGame = { ...editedGame!, name: (event.target as HTMLInputElement).value }" 
          id="game-name"
          class="col-span-4 mt-1"
        />
      </div>
    </div>

    <div class="flex gap-6 justify-end items-center">
      <Button :disabled="game == editedGame" @click="updateGame()">
        <Save class="w-4 h-4 mr-2" /> Updaten
      </Button>
      <Dialog>
        <DialogTrigger as-child>
          <Button variant="destructive">
            <Trash :size=20 class="mr-2"/> Verwijderen
          </Button>
        </DialogTrigger>
        <DialogContent class="sm:max-w-[425px]">
          <DialogTitle>Game Verwijderen</DialogTitle>
          <DialogDescription>
            Je staat op het punt om een game, inclusief gekoppelde items, te verwijderen <br>
            Weet je het zeker?
          </DialogDescription>

          <DialogFooter>
            <DialogClose as-child>
              <Button>
                Nee
              </Button>
            </DialogClose>
            <Button variant="outline" @click="deleteGame()">
              Ja
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  </GameLayout>
</template>