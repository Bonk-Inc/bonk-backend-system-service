<script setup lang="ts">
import GameLayout from '@/components/layout/GameLayout.vue';
import { Button } from '@/components/ui/button';
import { Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogTitle, DialogTrigger } from '@/components/ui/dialog';
import { useToast } from '@/components/ui/toast';
import type { ApiService } from '@/lib/ApiService';
import { Copy, Trash } from 'lucide-vue-next';
import { inject } from 'vue';
import { useRoute, useRouter } from 'vue-router';

const route = useRoute();
const router = useRouter();
const apiService = inject<ApiService>('api');
const { toast } = useToast();

const gameId = route.params.gameId as string;

const copyGameId = async () => {
  try {
    await navigator.clipboard.writeText(gameId);

    toast({
      description: 'Game ID gekopieerd!'
    });
  } catch (e: any) {
    console.error(e.message);
  }
}

const deleteGame = async() => {
  try {
    await apiService?.delete(`api/game/${gameId}/`);
    router.push({ name: 'app_home' });
  } catch(e: any) {
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
      <div class="py-4">
        <p class="font-bold">Game ID:</p>
        <div class="flex justify-between items-center">
          <p>{{gameId}}</p>
          <Button variant="ghost" size="icon">
            <Copy :size="20" @click="copyGameId()" />
          </Button>
        </div>
      </div>
    </div>

    <div class="flex justify-end items-center">
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