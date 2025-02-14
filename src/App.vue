<script setup lang="ts">
import { useAppMenu } from './composables/useAppMenu';
import { useBlocks } from './composables/useBlocks';
import AppNav from './components/AppNav.vue';

useAppMenu();
const { blocks, loadBlocks } = useBlocks();
</script>

<template>
  <div class="h-screen flex flex-col bg-white dark:bg-gray-900">
    <AppNav @open-file="loadBlocks" />

    <main class="flex-1 flex flex-col justify-center items-center">
    </main>
  </div>
</template>
