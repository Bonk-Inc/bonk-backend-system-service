<script setup lang="ts">
import { DatabaseZap } from 'lucide-vue-next';
import { NavigationMenu, NavigationMenuItem, NavigationMenuLink, NavigationMenuList, navigationMenuTriggerStyle } from './ui/navigation-menu';
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
            <NavigationMenuLink href="/swagger" :class="navigationMenuTriggerStyle()">
              API Documentation
            </NavigationMenuLink>
          </NavigationMenuItem>
        </NavigationMenuList>
      </NavigationMenu>
      </div>
  </header>

</template>