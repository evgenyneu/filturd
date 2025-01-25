<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { register, ShortcutEvent } from '@tauri-apps/plugin-global-shortcut';
import { readText } from '@tauri-apps/plugin-clipboard-manager'

const greetMsg = ref("");
const name = ref("");
const itemDescription = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}

const key = 'CommandOrControl+/'

register(key, async (event: ShortcutEvent) => {
  if (event.state !== 'Pressed') { return; }
  await invoke('simulate_copy');
  let textFromClipboard = await readText();

  if (textFromClipboard.startsWith('Item Class:')) {
    itemDescription.value = textFromClipboard;
    // invoke('play_sound', { file: 'camera_snap1.mp3' });
    invoke('play_sound', { file: '785640__litesaturation__cinematic-trailer-15-sec.mp3' }).then(() =>
      console.log('Sound play completed!')
    );
  }
});

</script>

<template>
  <main class="m-0 h-screen flex flex-col justify-center items-center">
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
</template>
