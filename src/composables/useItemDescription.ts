import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { register, ShortcutEvent, isRegistered } from '@tauri-apps/plugin-global-shortcut';
import { getCurrentWindow } from '@tauri-apps/api/window';

export function useItemDescription() {
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

  return {
    itemDescription
  }
}
