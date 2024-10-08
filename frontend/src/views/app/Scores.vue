<script setup lang="ts">
import DataTable from '@/components/DataTable.vue';
import GameLayout from '@/components/layout/GameLayout.vue';
import { Button } from '@/components/ui/button';
import { Checkbox } from '@/components/ui/checkbox';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { useToast } from '@/components/ui/toast';
import { ApiService } from '@/lib/ApiService';
import type { Level, Score } from '@/lib/Models';
import type { RowSelectionState, ColumnDef } from '@tanstack/vue-table';
import { format, parseISO } from 'date-fns';
import { ArrowUpDown, Eye, EyeOff, Pencil, Plus, Trash2 } from 'lucide-vue-next';
import { h, inject, onMounted, ref } from 'vue';
import { RouterLink, useRoute } from 'vue-router';

const route = useRoute();
const apiService = inject<ApiService>('api')
const gameId = route.params.gameId as string;
const { toast } = useToast();

const scores = ref<Score[]>([]);
const levels = ref<Level[]>([]);
const selectedScores = ref<RowSelectionState>({});

const columns: ColumnDef<Score>[] = [
  {
    id: 'select',
    header: ({ table }) => h(Checkbox, {
      'checked': table.getIsAllRowsSelected(),
      'onUpdate:checked': value => table.toggleAllPageRowsSelected(!!value)
    }),
    cell: ({ row }) => h(Checkbox, {
      'checked': row.getIsSelected(),
      'onUpdate:checked': value => row.toggleSelected(!!value),
    }),
    enableSorting: false,
    enableHiding: false,
  },
  {
    accessorKey: 'username',
    header: 'Username',
    cell: ({ row }) => h('p', {}, row.getValue('username')),
    enableHiding: false
  },
  {
    accessorKey: 'score',
    header: ({ column }) => {
      return h(Button, {
        variant: 'link',
        class: 'px-0',
        onClick: () => column.toggleSorting(column.getIsSorted() === 'asc'),
      }, () => ['Score', h(ArrowUpDown, { class: 'ml-2 h-4 w-4' })])
    },
    cell: ({ row }) => h('p', {}, row.getValue('score')),
    enableHiding: false,
  },
  {
    accessorKey: 'created_at',
    header: 'Gezet op',
    cell: ({ row }) => {
      const date = parseISO(row.getValue('created_at'))
      return h('p', {}, format(date, 'yyyy-MM-dd HH:mm:ss'))
    }
  },
  {
    accessorKey: 'is_hidden',
    header: 'Verborgen',
    cell: ({ row }) => {
      const hidden = row.original.is_hidden;
      const icon = hidden ? EyeOff : Eye;
      return h(icon)
    }
  },
  {
    accessorKey: 'actions',
    header: '',
    cell: ({ row }) => h(
      RouterLink,
      { to: { name: 'add_score', query: { gameId, scoreId: row.original.id } } },
      () => h(Pencil)
    )
  }
];

onMounted(async () => {
  try {
    const responseScores = await apiService?.get<Score[]>(`api/score/game/${gameId}/?hidden=true`);
    const responseLevels = await apiService?.get<Level[]>(`api/level/game/${gameId}/`);

    scores.value = responseScores?.data!;
    levels.value = responseLevels?.data!;
  } catch (e: unknown) {
    toast({
      title: 'Er ging wat fout :(',
      variant: 'destructive',
      description: 'Er is wat fout gegaan tijdens het ophalen van de data'
    });
  }
})

const filterLevel = async (level: string) => {
  const url = level.length > 0
    ? `api/score/level/${level}/?hidden=true`
    : `api/score/game/${gameId}/?hidden=true`;

  try {
    const response = await apiService?.get<Score[]>(url);
    scores.value = response?.data!;
  } catch (e: unknown) {
    toast({
      title: 'Er ging wat fout :(',
      variant: 'destructive',
      description: 'Er is wat fout gegaan tijdens het ophalen van de scores'
    });
  }
}

const updateVisibility = async (score: Score) => {
  try {
    const response = await apiService?.put<Score>(`api/score/${score.id}/`, JSON.stringify(score));
    const index = scores.value.findIndex(s => s.id === response?.data.id);
    scores.value.splice(index, 1, response?.data as Score);
  } catch (e: unknown) {

  }
};

const deleteSelectedRows = async () => {
  const ids = Object.keys(selectedScores.value)
    .map(key => parseInt(key))
    .map(i => scores.value[i].id)
    .join(',');

  try {
    await apiService?.delete(`api/score/(${ids})/`);
    const response = await apiService?.get<Score[]>(`api/score/game/${gameId}/?hidden=true`);

    scores.value = response?.data!;
    selectedScores.value = {};
  } catch(e: unknown) {
    toast({
      title: 'Er ging wat fout :(',
      variant: 'destructive',
      description: 'Er is wat fout gegaan tijdens het verwijderen van de scores'
    });
  }
}
</script>

<template>
  <GameLayout>
    <div class="pt-2 pb-4 flex justify-between items-center">
      <Button 
        v-if="Object.keys(selectedScores).length > 0"
        @click="deleteSelectedRows()"
        variant="ghost"
        size="icon">
        <Trash2 :size="24" class="text-destructive" />
      </Button>
      <div class="flex items-center justify-start" v-else>
        <div class="flex items-center">
          <Select class="mt-2" @update:model-value="$event => filterLevel($event)">
            <SelectTrigger class="w-[160px]">
              <SelectValue placeholder="Filter op Level" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem v-for="level in levels" :key="level.id" :value="level.id">
                {{ level.name }}
              </SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>
      <Button as-child>
        <RouterLink :to="{ name: 'add_score', query: { gameId } }">
          <Plus class="mr-2" /> Toevoegen
        </RouterLink>
      </Button>
    </div>
    <DataTable 
      :data="scores" 
      :columns="columns" 
      v-model:selected="selectedScores"
    />
  </GameLayout>
</template>