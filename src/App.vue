<script setup lang="ts">
import { useAppMenu } from './composables/useAppMenu';
import { useBlocks } from './composables/useBlocks';
import { items } from './utils/items';
import AppNav from './components/AppNav.vue';
import FilterTable from './components/FilterTable.vue';
import { useFontSize } from './composables/useFontSize';

const { blocks, loadBlocks, usedItemNames } = useBlocks();
const { fontSizeMultiplier, cycleFontSize } = useFontSize();

useAppMenu(loadBlocks);
</script>

<template>
  <div class="h-screen flex flex-col bg-white dark:bg-gray-900">
    <AppNav @open-file="loadBlocks" @cycle-font-size="cycleFontSize" />

    <main class="flex-1 overflow-auto bg-white dark:bg-gray-950">
      <FilterTable v-if="blocks.length" :blocks="blocks" :used-item-names="usedItemNames" :items="items"
        :font-size-multiplier="fontSizeMultiplier" />
    </main>
  </div>
</template>
