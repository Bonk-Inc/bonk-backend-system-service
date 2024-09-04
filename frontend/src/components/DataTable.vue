<script setup lang="ts" generic="TData, TValue">
import { FlexRender, getCoreRowModel, getPaginationRowModel, getSortedRowModel, useVueTable, type ColumnDef, type SortingState, type Table as VueTable } from '@tanstack/vue-table';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from './ui/table';
import { ref, watch } from 'vue';
import { valueUpdater } from '@/lib/utils';

const props = defineProps<{
  data: TData[],
  columns: ColumnDef<TData, TValue>[]
}>();

const sorting = ref<SortingState>();
const table = ref<VueTable<TData>>(createTable());

watch(
  () => props.data,
  () => table.value = createTable(),
  { deep: true }
);

function createTable(): VueTable<TData> {
  return useVueTable({
    get data() { return props.data },
    get columns() { return props.columns },
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    onSortingChange: updaterOrValue => valueUpdater(updaterOrValue, sorting),
    state: {
      get sorting() { return sorting.value }
    }
  })
};
</script>

<template>
  <div class="rounded-md border">
    <Table>
      <TableHeader>
        <TableRow v-for="headerGroup in table.getHeaderGroups()" :key="headerGroup.id">
          <TableHead v-for="header in headerGroup.headers" :key="header.id">
            <FlexRender v-if="!header.isPlaceholder" :render="header.column.columnDef.header"
              :props="header.getContext()" />
          </TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <template v-if="table.getRowCount()">
          <TableRow v-for="row in table.getRowModel().rows" :key="row.id"
            :data-state="row.getIsSelected() ? 'selected' : undefined">
            <TableCell v-for="cell in row.getVisibleCells()" :key="cell.id">
              <FlexRender :render="cell.column.columnDef.cell" :props="cell.getContext()" />
            </TableCell>
          </TableRow>
        </template>

        <template v-else>
          <TableRow>
            <TableCell :colspan="columns.length" class="h-24 text-center">
              Geen resulaten.
            </TableCell>
          </TableRow>
        </template>
      </TableBody>
    </Table>
  </div>
</template>