<script setup lang="ts">
import { useAppMenu } from './composables/useAppMenu';
import { useBlocks } from './composables/useBlocks';
import AppNav from './components/AppNav.vue';
import FilterTable from './components/FilterTable.vue';

useAppMenu();
const { blocks, loadBlocks } = useBlocks();
</script>

<template>
  <div class="h-screen flex flex-col bg-white dark:bg-gray-900">
    <AppNav @open-file="loadBlocks" />

    <main class="flex-1 p-4 overflow-auto">
      <FilterTable v-if="blocks.length" :blocks="blocks" />
    </main>
  </div>
</template>
