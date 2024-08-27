<script setup lang="ts">
import { House, Map, Settings, Tally5 } from 'lucide-vue-next';
import { Tabs, TabsList, TabsTrigger } from '../ui/tabs';
import MainLayout from './MainLayout.vue';
</script>

<template>
  <MainLayout class="p-0">
    <Tabs class="w-full border-border border-b" default-value="home">
      <TabsList>
        <TabsTrigger value="home">
          <House class="mr-2"/> Home
        </TabsTrigger>
        <TabsTrigger value="levels">
          <Map class="mr-2" /> Levels
        </TabsTrigger>
        <TabsTrigger value="scores">
          <Tally5 class="mr-2"/> scores
        </TabsTrigger>
        <TabsTrigger value="settings">
          <Settings class="mr-2"/> Instellingen
        </TabsTrigger>
      </TabsList>
    </Tabs>
    <div class="p-4">
      <slot></slot>
    </div>
  </MainLayout>
</template>