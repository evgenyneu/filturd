<script setup lang="ts">
import { useAppMenu } from './composables/useAppMenu';
import { useBlocks } from './composables/useBlocks';
import { items } from './utils/items';
import AppNav from './components/AppNav.vue';
import FilterTable from './components/FilterTable.vue';

const { blocks, loadBlocks, usedItemNames } = useBlocks();
useAppMenu(loadBlocks);
</script>

<template>
  <div class="h-screen flex flex-col bg-white dark:bg-gray-900">
    <AppNav @open-file="loadBlocks" />

    <main class="flex-1 px-4 overflow-auto">
      <FilterTable v-if="blocks.length" :blocks="blocks" :used-item-names="usedItemNames" :items="items" />
    </main>
  </div>
</template>
