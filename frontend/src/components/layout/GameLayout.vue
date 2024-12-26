<script setup lang="ts">
import { House, Map, Settings, Tally5, Users } from 'lucide-vue-next';
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
          <RouterLink 
            class="flex items-center"
            exact-active-class="text-blue-500"
            :to="{ name: 'game_home', params: { gameId: route.params.gameId } }"
          >
            <House class="mr-2"/> Home
          </RouterLink>
        </TabsTrigger>
        <TabsTrigger value="levels">
          <RouterLink 
            class="flex items-center" 
            exact-active-class="text-blue-500"
            :to="{ name: 'game_levels', params: { gameId: route.params.gameId } }"
          >
            <Map class="mr-2" /> Levels
          </RouterLink>
        </TabsTrigger>
        <TabsTrigger value="scores">
          <RouterLink 
            class="flex items-center" 
            exact-active-class="text-blue-500"
            :to="{ name: 'game_scores', params: { gameId: route.params.gameId } }"
          >
            <Tally5 class="mr-2"/> Scores
          </RouterLink>
        </TabsTrigger>
        <TabsTrigger value="users">
          <RouterLink
            class="flex items-center" 
            exact-active-class="text-blue-500"
            :to="{ name: 'game_users', params: { gameId: route.params.gameId } }"
          >
          <Users class="mr-2"/> Gebruikers
          </RouterLink>
        </TabsTrigger>
        <TabsTrigger value="settings">
          <RouterLink 
            class="flex items-center" 
            exact-active-class="text-blue-500"
            :to="{ name: 'game_settings', params: { gameId: route.params.gameId } }"
          >
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