<script setup lang="ts">
import GameLayout from '@/components/layout/GameLayout.vue';
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert';
import { Button } from '@/components/ui/button';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip';
import { ApiService } from '@/lib/ApiService';
import { type Level } from '@/lib/Models';
import { Check, Copy, Trash } from 'lucide-vue-next';
import { inject, onMounted, ref } from 'vue';
import { useRoute } from 'vue-router';

const route = useRoute();
const apiService = inject<ApiService>('api');
const gameId = route.params.gameId as string;

const levels = ref<Level[]>([]);
const errors = ref<string>('');
const info = ref<string>('');

onMounted(async () => {
  await fetchLevels(gameId);
});

const fetchLevels = async (id: string) => {
  try {
    const response = await apiService?.get<Level[]>(`api/level/game/${id}/`);
    levels.value = response?.data ?? [];
  } catch(e: any) {
    errors.value = e.message;
  }
};

const copyLevelId = async (id: string) => {
  try {
    await navigator.clipboard.writeText(id);
  } catch (e: any) {
    console.error(e.message);
  }
}

const deleteLevel = async (id: string) => {
  try {
    await apiService?.delete(`api/level/${id}/`);
    info.value = 'Level verwijderd';

    setTimeout(() => info.value = '', 5_000);
    await fetchLevels(gameId);
  } catch (e: any) {
    errors.value = e.message;
    setTimeout(() => errors.value = '', 5_000);
  }
}
</script>

<template>
  <GameLayout>
    <Alert v-if="info" class="absolute z-10 selection:top-20 inset-x-0 w-full max-w-xl mx-auto">
      <Check class="w-4 h-4" />
      <AlertTitle>Gelukt!</AlertTitle>
      <AlertDescription>{{ info }}</AlertDescription>
    </Alert>

    <Table>
      <TableHeader>
        <TableRow>
          <TableHead>Naam</TableHead>
          <TableHead></TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow v-for="level in levels">
          <TableCell>{{ level.name }}</TableCell>
          <TableCell>
            <div class="flex items-center justify-end">
              <TooltipProvider>
                <Tooltip>
                  <TooltipTrigger as-child>
                    <Button variant="ghost" size="icon" @click="copyLevelId(level.id)">
                      <Copy :size="20" />
                    </Button>
                  </TooltipTrigger>
                  <TooltipContent>
                    <p>Kopieer Level ID</p>
                  </TooltipContent>
                </Tooltip>
              </TooltipProvider>
             
              <Button variant="ghost" size="icon" class="ml-4" @click="deleteLevel(level.id)">
                <Trash :size="20" />
              </Button>
            </div>
          </TableCell>
        </TableRow>
      </TableBody>
    </Table>
  </GameLayout>
</template>