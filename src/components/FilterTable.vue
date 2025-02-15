<script setup lang="ts">
import type { Block } from '../../src-tauri/bindings/Block';
import type { Items } from '../types/items';
import { ref, computed, watch } from 'vue';

const props = defineProps<{
  blocks: Block[]
  usedItemNames: string[]
  items: Items
}>();

type SortKey = string;
type Direction = 'asc' | 'desc' | null;

const sortKey = ref<SortKey>('order');
const sortDirection = ref<Direction>('asc');

const sortedBlocks = computed(() => {
  if (!sortKey.value) return props.blocks;

  return [...props.blocks].sort((a, b) => {
    const modifier = sortDirection.value === 'asc' ? 1 : -1;

    if (sortKey.value === 'order') {
      return (a.order - b.order) * modifier;
    }

    return (a.name.localeCompare(b.name)) * modifier;
  });
});

function handleSort(key: SortKey) {
  if (sortKey.value !== key) {
    sortKey.value = key;
    sortDirection.value = 'asc';
  } else {
    sortDirection.value = sortDirection.value === 'asc' ? 'desc' : 'asc';
  }
}

watch(() => props.blocks, () => {
  sortKey.value = 'order';
  sortDirection.value = 'asc';
});

const sortedItemColumns = computed(() => {
  return [...props.usedItemNames].sort((a, b) => {
    const countA = props.items[a]?.frequency || 0;
    const countB = props.items[b]?.frequency || 0;
    return countB - countA;
  });
});

</script>

<template>
  <table class="w-full border-collapse bg-white/5 dark:bg-poe-bg/20">
    <thead>
      <tr class="border-b border-gray-200 dark:border-poe-border">
        <th @click="handleSort('order')" class="w-fit whitespace-nowrap py-2 px-1 text-center font-normal group
                   border-r border-gray-200 dark:border-poe-border">
          <button
            class="flex items-center justify-center w-full hover:underline cursor-pointer text-gray-600 dark:text-poe-text-400">
            <span>#</span>
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
              stroke="currentColor" class="size-4" :class="{ 'invisible': sortKey !== 'order' }">
              <path v-show="sortDirection === 'asc'" stroke-linecap="round" stroke-linejoin="round"
                d="M8.25 6.75 12 3m0 0 3.75 3.75M12 3v18" />
              <path v-show="sortDirection === 'desc'" stroke-linecap="round" stroke-linejoin="round"
                d="M15.75 17.25 12 21m0 0-3.75-3.75M12 21V3" />
            </svg>
          </button>
        </th>
        <th @click="handleSort('name')" class="py-2 px-1 text-center font-normal group
                   border-r border-gray-200 dark:border-poe-border">
          <button
            class="flex items-center justify-center w-full hover:underline cursor-pointer text-gray-600 dark:text-poe-text-400">
            <span>Type</span>
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
              stroke="currentColor" class="size-4" :class="{ 'invisible': sortKey !== 'name' }">
              <path v-show="sortDirection === 'asc'" stroke-linecap="round" stroke-linejoin="round"
                d="M8.25 6.75 12 3m0 0 3.75 3.75M12 3v18" />
              <path v-show="sortDirection === 'desc'" stroke-linecap="round" stroke-linejoin="round"
                d="M15.75 17.25 12 21m0 0-3.75-3.75M12 21V3" />
            </svg>
          </button>
        </th>
        <th v-for="item in sortedItemColumns" :key="item" @click="handleSort(item)" class="py-2 px-1 text-center font-normal whitespace-nowrap group
                   border-r border-gray-200 dark:border-poe-border
                   last:border-r-0">
          <button
            class="flex items-center justify-center w-full hover:underline cursor-pointer text-gray-600 dark:text-poe-text-400">
            <span>{{ item }}</span>
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
              stroke="currentColor" class="size-4" :class="{ 'invisible': sortKey !== item }">
              <path v-show="sortDirection === 'asc'" stroke-linecap="round" stroke-linejoin="round"
                d="M8.25 6.75 12 3m0 0 3.75 3.75M12 3v18" />
              <path v-show="sortDirection === 'desc'" stroke-linecap="round" stroke-linejoin="round"
                d="M15.75 17.25 12 21m0 0-3.75-3.75M12 21V3" />
            </svg>
          </button>
        </th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="block in sortedBlocks" :key="block.order" class="border-b border-gray-200/50 dark:border-poe-border/50
                 hover:bg-gray-50 dark:hover:bg-poe-border/30
                 transition-colors duration-150">
        <td class="py-2 px-1 text-gray-900 dark:text-poe-text-400
                   border-r border-gray-200/50 dark:border-poe-border/50">
          {{ block.order }}
        </td>
        <td class="py-2 px-1
                   border-r border-gray-200/50 dark:border-poe-border/50">
          <span class="inline-block px-2 py-1 rounded" :class="{
            'bg-green-700 text-green-100': block.name === 'Show',
            'bg-red-600 text-red-100': block.name === 'Hide'
          }">
            {{ block.name.toLowerCase() }}
          </span>
        </td>
        <td v-for="item in sortedItemColumns" :key="item" class="py-2 px-1
                   border-r border-gray-200/50 dark:border-poe-border/50
                   last:border-r-0">
          <template v-if="block.items.some(i => i.name === item)">
            <div v-for="blockItem in block.items.filter(i => i.name === item)"
              :key="`${blockItem.name}-${blockItem.params.join('-')}`" class="mb-1 last:mb-0">
              <span v-for="param in blockItem.params" :key="param" class="inline-block px-2 py-1 rounded mr-1
                           bg-gray-200/50 dark:bg-poe-border/20
                           text-gray-700 dark:text-poe-text-400">
                {{ param }}
              </span>
            </div>
          </template>
        </td>
      </tr>
    </tbody>
  </table>
</template>
