<script setup lang="ts">
import { useAppMenu } from './composables/useAppMenu';
import { useItemDescription } from './composables/useItemDescription';
import AppNav from './components/AppNav.vue';

useAppMenu();
const { itemDescription } = useItemDescription();
</script>

<template>
  <div class="h-screen flex flex-col bg-white dark:bg-gray-900">
    <AppNav />

    <main class="flex-1 flex flex-col justify-center items-center">
      <pre class="text-left whitespace-pre-wrap text-gray-900 dark:text-white">{{ itemDescription }}</pre>
    </main>
  </div>
</template>
