<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { register, ShortcutEvent, isRegistered } from '@tauri-apps/plugin-global-shortcut';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useAppMenu } from './composables/useAppMenu';
import AppNav from './components/AppNav.vue';

const itemDescription = ref("");

const key = 'CommandOrControl+1'

useAppMenu();

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
</script>

<template>
  <div class="h-screen flex flex-col bg-white dark:bg-gray-900">
    <AppNav />

    <main class="flex-1 flex flex-col justify-center items-center">
      <pre class="text-left whitespace-pre-wrap text-gray-900 dark:text-white">{{ itemDescription }}</pre>
    </main>
  </div>
</template>
