<script setup lang="ts">
import DataTable from '@/components/DataTable.vue';
import GameLayout from '@/components/layout/GameLayout.vue';
import { Button } from '@/components/ui/button';
import { toast } from '@/components/ui/toast';
import type { ApiService } from '@/lib/ApiService';
import type { User } from '@/lib/Models/User';
import type { ColumnDef } from '@tanstack/vue-table';
import { format, parseISO } from 'date-fns';
import { Trash } from 'lucide-vue-next';
import { h, inject, onMounted, ref } from 'vue';
import { useRoute } from 'vue-router';

const route = useRoute();
const apiService = inject<ApiService>('api')
const gameId = route.params.gameId as string;

const users = ref<User[]>([]);

onMounted(async () => {
  await fetchUsers(gameId);
});

const fetchUsers = async (id: string) => {
  try {
    const response = await apiService?.get<User[]>(`api/user/game/${id}`);
    users.value = response?.data ?? [];
  } catch(e: unknown) {
    toast({
      title: 'Er ging wat fout :(',
      variant: 'destructive',
      description: 'Er is wat fout gegaan tijdens het ophalen van de gebruikers'
    });
  }
};

const deleteUser = async (user: User) => {
  try {
    if(!confirm(`Wil je gebruiker ${user.name} verwijderen?`))
      return;

    await apiService?.delete(`api/user/${user.id}`);
    await fetchUsers(gameId); 
  } catch(e: unknown) {
    toast({
      title: 'Er ging wat fout :(',
      variant: 'destructive',
      description: 'Er is wat fout gegaan tijdens het verwijderen van de user'
    });
  }
}

const columns: ColumnDef<User>[] = [
  {
    accessorKey: 'name',
    header: 'Naam',
    cell: ({ row }) => h('p', {}, row.getValue('name')),
    enableHiding: false
  },
  {
    accessorKey: 'created_at',
    header: 'Aangemaakt op',
    cell: ({ row }) => {
      const date = parseISO(row.getValue('created_at'))
      return h('p', {}, format(date, 'yyyy-MM-dd HH:mm:ss'))
    }
  },
  {
    accessorKey: 'actions',
    header: '',
    cell: ({ row }) => h(
      Button,
      { variant: 'ghost', size: 'icon', onClick: () => deleteUser(row.original) },
      () => h(Trash, { size: 20 })
    )
  }
];

</script>

<template>
  <GameLayout>
    <DataTable
      :data="users"
      :columns="columns"
    />
  </GameLayout>
</template>