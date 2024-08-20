<script setup lang="ts">
import Header from '../Header.vue';
import Sidebar from '../Sidebar.vue';
</script>

<template>
  <Header />
  <div class="flex w-full content items-stretch">
    <Sidebar :games="[]" />
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