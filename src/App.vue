<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { register, ShortcutEvent, isRegistered } from '@tauri-apps/plugin-global-shortcut';
import { getCurrentWindow } from '@tauri-apps/api/window';

const itemDescription = ref("");

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

</script>

<template>
  <div class="h-screen flex flex-col bg-white dark:bg-gray-900">
    <nav
      class="p-4 bg-gray-100 dark:bg-gray-800 flex justify-between items-center border-b border-gray-200 dark:border-gray-700">
      <button @click="openFile" class="rounded-lg border border-transparent px-4 py-2 text-base font-medium
               bg-white text-gray-900 dark:bg-poe-bg dark:text-poe-text-50
               hover:border-poe-hover-600 hover:text-poe-hover-600
               dark:hover:border-poe-hover-300 dark:hover:text-poe-hover-300
               active:bg-gray-200 dark:active:bg-poe-border
               shadow-sm cursor-pointer transition-colors duration-250
               focus:outline-none flex items-center gap-2">
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
          class="w-5 h-5">
          <path stroke-linecap="round" stroke-linejoin="round"
            d="M3 7v10a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-6l-2-2H5a2 2 0 0 0-2 2z" />
          <path stroke-linecap="round" stroke-linejoin="round" d="M9 14l3-3m0 0l3 3m-3-3v7" />
        </svg>
        Open
      </button>
    </nav>

    <main class="flex-1 flex flex-col justify-center items-center">
      <pre class="text-left whitespace-pre-wrap text-gray-900 dark:text-white">{{ itemDescription }}</pre>
    </main>
  </div>
</template>
