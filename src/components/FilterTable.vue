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
  const conditionalItems = props.usedItemNames.filter(name => props.items[name]?.is_condition);
  const actionItems = props.usedItemNames.filter(name => !props.items[name]?.is_condition);

  // Sort each group by frequency
  const sortByFrequency = (a: string, b: string) => {
    const countA = props.items[a]?.frequency || 0;
    const countB = props.items[b]?.frequency || 0;
    return countB - countA;
  };

  // Return conditional items first, then non-conditional, both sorted by frequency
  return [
    ...conditionalItems.sort(sortByFrequency),
    ...actionItems.sort(sortByFrequency)
  ];
});

</script>

<template>
  <table class="w-full border-collapse bg-white dark:bg-gray-900">
    <thead>
      <tr class="border-b border-gray-200 dark:border-gray-800">
        <th @click="handleSort('order')" class="w-fit whitespace-nowrap py-2 px-1 text-center font-normal group
                   border-r border-gray-200 dark:border-gray-800">
          <button
            class="flex items-center justify-center w-full hover:underline hover:text-amber-700 dark:hover:text-amber-300 cursor-pointer text-gray-800 dark:text-gray-400">
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
                   border-r border-gray-200 dark:border-gray-800">
          <button
            class="flex items-center justify-center w-full hover:underline hover:text-amber-700 dark:hover:text-amber-300 cursor-pointer text-gray-800 dark:text-gray-400">
            <span class="pl-2">Type</span>
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
                   border-r border-gray-200 dark:border-gray-800
                   last:border-r-0">
          <button
            class="flex items-center justify-center w-full hover:underline hover:text-amber-700 dark:hover:text-amber-300 cursor-pointer text-gray-800 dark:text-gray-400">
            <span class="pl-2">{{ item }}</span>
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
      <tr v-for="block in sortedBlocks" :key="block.order" class="border-b border-gray-200 dark:border-gray-800">
        <td class="py-2 px-1 text-gray-800 dark:text-gray-400
                   border-r border-gray-200 dark:border-gray-800 text-center">
          {{ block.order }}
        </td>
        <td class="py-2 px-1
                   border-r border-gray-200 dark:border-gray-800 text-center">
          <span class="inline-block px-2 py-1 rounded" :class="{
            'bg-green-700 text-green-100': block.name === 'Show',
            'bg-red-600 text-red-100': block.name === 'Hide'
          }">
            {{ block.name.toLowerCase() }}
          </span>
        </td>
        <td v-for="item in sortedItemColumns" :key="item" class="py-2 px-1
                   border-r border-gray-200 dark:border-gray-800
                   last:border-r-0 text-center">
          <template v-if="block.items[item]">
            <div v-for="(blockItem, index) in block.items[item]" :key="index" class="mb-1 last:mb-0">
              <span v-for="param in blockItem.params" :key="param" class="inline-block px-2 py-1 rounded mr-1 mb-1
                           bg-gray-100 dark:bg-gray-800
                           text-gray-800 dark:text-gray-400">
                {{ param }}
              </span>
            </div>
          </template>
        </td>
      </tr>
    </tbody>
  </table>
</template>
