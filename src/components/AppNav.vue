<script setup lang="ts">
import { useTheme } from '../composables/useTheme';

const { theme, updateTheme } = useTheme();
defineProps<{
  onOpenFile?: () => void
}>();
</script>

<template>
  <nav
    class="p-4 bg-gray-100 dark:bg-gray-900 flex justify-between items-center border-b border-gray-200 dark:border-gray-700">
    <button @click="onOpenFile" class="rounded-lg border px-4 py-2 text-base font-medium
             bg-white text-gray-800 dark:bg-gray-900 dark:text-gray-50
             border-transparent dark:border-gray-700
             hover:border-amber-700 hover:text-amber-700
             dark:hover:border-amber-300 dark:hover:text-amber-300
             active:bg-gray-200 dark:active:bg-gray-700
             shadow-sm cursor-pointer transition-colors duration-150
             focus:outline-none focus-visible:ring-2 focus-visible:ring-amber-700 dark:focus-visible:ring-amber-300
             flex items-center gap-2">
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
        class="size-6">
        <path stroke-linecap="round" stroke-linejoin="round"
          d="M12 6.042A8.967 8.967 0 0 0 6 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 0 1 6 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 0 1 6-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0 0 18 18a8.967 8.967 0 0 0-6 2.292m0-14.25v14.25" />
      </svg>
      Open Filter
    </button>

    <button @click="updateTheme(theme === 'dark' ? 'light' : 'dark')" class="rounded-lg border p-2 text-base font-medium
             bg-white text-gray-800 dark:bg-gray-900 dark:text-gray-50
             border-transparent dark:border-gray-700
             hover:border-amber-700 hover:text-amber-700
             dark:hover:border-amber-300 dark:hover:text-amber-300
             active:bg-gray-200 dark:active:bg-gray-700
             shadow-sm cursor-pointer transition-colors duration-150
             focus:outline-none focus-visible:ring-2 focus-visible:ring-amber-700 dark:focus-visible:ring-amber-300"
      :title="theme === 'dark' ? 'Switch to light mode' : 'Switch to dark mode'">
      <svg v-if="theme === 'dark'" xmlns="http://www.w3.org/2000/svg" class="size-5" fill="none" viewBox="0 0 24 24"
        stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
          d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
      </svg>
      <svg v-else xmlns="http://www.w3.org/2000/svg" class="size-5" fill="none" viewBox="0 0 24 24"
        stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
          d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
      </svg>
    </button>
  </nav>
</template>
