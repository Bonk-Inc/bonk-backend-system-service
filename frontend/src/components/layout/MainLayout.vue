<script setup lang="ts">
import { Skeleton } from '../ui/skeleton';
import Header from '../Header.vue';
import Sidebar from '../Sidebar.vue';
import type { HtmlHTMLAttributes } from 'vue';
import { cn } from '@/lib/utils';
import { Toaster } from '../ui/toast';

interface Props {
  class?: HtmlHTMLAttributes['class'];
}

const props = defineProps<Props>();
</script>

<template>
  <Toaster />
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

    <main :class="cn('p-4 w-full', props.class)">
      <slot></slot>
    </main>
  </div>
</template>

<style lang="css">
.content {
  min-height: calc(100vh - 3.5rem);
}
</style>