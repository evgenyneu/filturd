<script setup lang="ts">
import type { Block } from '../../src-tauri/bindings/Block';

defineProps<{
  blocks: Block[]
}>();
</script>

<template>
  <table class="w-full border-collapse bg-white/5 dark:bg-poe-bg/20">
    <thead>
      <tr class="border-b border-gray-200 dark:border-poe-border">
        <th class="w-fit whitespace-nowrap py-2 px-1 text-left font-normal text-gray-600 dark:text-poe-text-900">#</th>
        <th class="w-full py-2 px-1 text-left font-normal text-gray-600 dark:text-poe-text-900">Type</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="block in blocks" :key="block.order" class="border-b border-gray-200/50 dark:border-poe-border/50
                 hover:bg-gray-50 dark:hover:bg-poe-border/30
                 transition-colors duration-150">
        <td class="py-2 px-1 font-medium text-gray-900 dark:text-poe-text-400">{{ block.order }}</td>
        <td class="py-2 px-1 font-medium text-gray-900 dark:text-poe-text-400">{{ block.name }}</td>
      </tr>
    </tbody>
  </table>
</template>
