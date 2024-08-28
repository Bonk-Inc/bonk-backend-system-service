<script setup lang="ts">
import { House, Map, Settings, Tally5 } from 'lucide-vue-next';
import { Tabs, TabsList, TabsTrigger } from '../ui/tabs';
import MainLayout from './MainLayout.vue';
import { useRoute } from 'vue-router';

const route = useRoute();
</script>

<template>
  <MainLayout class="p-0">
    <Tabs class="w-full border-border border-b" default-value="home">
      <TabsList>
        <TabsTrigger value="home">
          <RouterLink class="flex items-center" :to="`${route.path}`">
            <House class="mr-2"/> Home
          </RouterLink>
        </TabsTrigger>
        <TabsTrigger value="levels">
          <RouterLink class="flex items-center" :to="`${route.path}/levels`">
            <Map class="mr-2" /> Levels
          </RouterLink>
        </TabsTrigger>
        <TabsTrigger value="scores">
          <RouterLink class="flex items-center" :to="`${route.path}/scores`">
            <Tally5 class="mr-2"/> scores
          </RouterLink>
        </TabsTrigger>
        <TabsTrigger value="settings">
          <RouterLink class="flex items-center" :to="`${route.path}/settings`">
            <Settings class="mr-2"/> Instellingen
          </RouterLink>
        </TabsTrigger>
      </TabsList>
    </Tabs>
    <div class="p-4">
      <slot></slot>
    </div>
  </MainLayout>
</template>