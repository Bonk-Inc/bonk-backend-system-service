<script setup lang="ts">
import { Skeleton } from '../ui/skeleton';
import Header from '../Header.vue';
import Sidebar from '../Sidebar.vue';

</script>

<template>
  <Header />
  <div class="flex w-full content items-stretch">
    <Suspense>     
      <Sidebar />
      
      <template #fallback>
        <aside class="w-60 border-border border-r px-4">
          <Skeleton class="h-5 my-4" />
          <Skeleton class="h-5 my-4" />
          <Skeleton class="h-5 my-4" />
        </aside>
      </template>
    </Suspense>

    <main class="p-4">
      <slot></slot>
    </main>
  </div>
</template>

<style lang="css">
.content {
  min-height: calc(100vh - 3.5rem);
}
</style>