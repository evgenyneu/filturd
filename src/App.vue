<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { register, ShortcutEvent, isRegistered } from '@tauri-apps/plugin-global-shortcut';
import { getCurrentWindow } from '@tauri-apps/api/window';

const greetMsg = ref("");
const name = ref("");
const itemDescription = ref("");
const isDark = ref(false);

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}

const key = 'CommandOrControl+1'

async function didPressCopyShortcut(event: ShortcutEvent) {
  if (event.state !== 'Pressed') { return; }
  let descriptionFromClipboard: string | null = await invoke('copy_item_description_under_cursor');
  if (!descriptionFromClipboard) { return; }
  itemDescription.value = descriptionFromClipboard;
  invoke('play_sound', { file: 'camera_snap1.mp3' });
  const window = getCurrentWindow();
  await window.setFocus();
}

isRegistered(key).then((isRegistered) => {
  if (isRegistered) { return }

  register(key, didPressCopyShortcut);
});

async function openFile() {
  // TODO: Implement file opening logic
}

function toggleDarkMode() {
  isDark.value = !isDark.value;
  document.documentElement.classList.toggle('dark');
}

onMounted(() => {
  // Check system preference on load
  isDark.value = window.matchMedia('(prefers-color-scheme: dark)').matches;

  if (isDark.value) {
    document.documentElement.classList.add('dark');
  }
});

</script>

<template>
  <div class="h-screen flex flex-col">
    <nav class="p-4 bg-gray-100 dark:bg-gray-800 flex justify-between items-center">
      <button @click="openFile"
        class="rounded-lg border border-transparent px-4 py-2 text-base font-medium text-gray-900 bg-white shadow-sm cursor-pointer transition-colors duration-250 hover:border-blue-600 active:bg-gray-200 focus:outline-none dark:text-white dark:bg-gray-900/60 dark:active:bg-gray-900/40">
        Open
      </button>

      <button @click="toggleDarkMode" class="p-2 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors">
        <svg v-if="isDark" xmlns="http://www.w3.org/2000/svg" class="w-6 h-6 text-gray-800 dark:text-white" fill="none"
          viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
            d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
        </svg>
        <svg v-else xmlns="http://www.w3.org/2000/svg" class="w-6 h-6 text-gray-800 dark:text-white" fill="none"
          viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
            d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
        </svg>
      </button>
    </nav>

    <main class="flex-1 flex flex-col justify-center items-center">
      <h1 class="text-center">Welcome to Tauri + Vue</h1>

      <div class="flex">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" class="h-24 p-6 transition-all duration-750 hover:drop-shadow-[0_0_2em_#747bff]"
            alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" class="h-24 p-6 transition-all duration-750 hover:drop-shadow-[0_0_2em_#24c8db]"
            alt="Tauri logo" />
        </a>
        <a href="https://vuejs.org/" target="_blank">
          <img src="./assets/vue.svg" class="h-24 p-6 transition-all duration-750 hover:drop-shadow-[0_0_2em_#249b73]"
            alt="Vue logo" />
        </a>
      </div>

      <p class="mb-4">Click on the Tauri, Vite, and Vue logos to learn more.</p>

      <form class="flex justify-center mb-4" @submit.prevent="greet">
        <input id="greet-input" v-model="name" placeholder="Enter a name..."
          class="mr-2 rounded-lg border border-transparent px-4 py-2 text-base font-medium text-gray-900 bg-white shadow-sm transition-colors duration-250 hover:border-blue-600 focus:outline-none dark:text-white dark:bg-gray-900/60" />

        <button type="submit"
          class="rounded-lg border border-transparent px-4 py-2 text-base font-medium text-gray-900 bg-white shadow-sm cursor-pointer transition-colors duration-250 hover:border-blue-600 active:bg-gray-200 focus:outline-none dark:text-white dark:bg-gray-900/60 dark:active:bg-gray-900/40">
          Greet
        </button>
      </form>
      <p>{{ greetMsg }}</p>
      <pre class="text-left whitespace-pre-wrap">{{ itemDescription }}</pre>
    </main>
  </div>
</template>
