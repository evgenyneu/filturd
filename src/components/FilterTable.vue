<script setup lang="ts">
import type { Block } from '../../src-tauri/bindings/Block';

defineProps<{
  blocks: Block[]
}>();
</script>

<template>
  <table class="w-full border-collapse">
    <thead>
      <tr class="bg-gray-100 dark:bg-gray-800">
        <th class="p-2 text-left">#</th>
        <th class="p-2 text-left">Type</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="block in blocks" :key="block.order" class="border-t border-gray-200 dark:border-gray-700">
        <td class="p-2">{{ block.order }}</td>
        <td class="p-2">{{ block.name }}</td>
      </tr>
    </tbody>
  </table>
</template>
