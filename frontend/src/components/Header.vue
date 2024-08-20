<script setup lang="ts">
import { DatabaseZap } from 'lucide-vue-next';
import { NavigationMenu, NavigationMenuItem, NavigationMenuLink, NavigationMenuList, navigationMenuTriggerStyle } from './ui/navigation-menu';

const apiUrl = import.meta.env.VITE_APP_API_URL;
</script>

<template>
  <header class="h-14 w-full border-b border-border">
    <div class="flex h-full justify-between items-center px-6">
      <div class="flex items-center">
        <DatabaseZap :size="28" class="mr-4" />
        <p class="medium text-xl">Bonk Inc Backend</p>
      </div>
      <NavigationMenu>
        <NavigationMenuList>
          <NavigationMenuItem>
            <NavigationMenuLink :href="`${apiUrl}/swagger/`" :class="navigationMenuTriggerStyle()">
              API Documentation
            </NavigationMenuLink>
          </NavigationMenuItem>
        </NavigationMenuList>
      </NavigationMenu>
      </div>
  </header>

</template>