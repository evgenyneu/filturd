import { ref } from "vue";
import { openFile } from "../utils/fileOpener";
import type { Block } from "../../src-tauri/bindings/Block";
import { items_used_in_blocks } from "../utils/items";

export function useBlocks() {
  const blocks = ref<Block[]>([]);
  const usedItemNames = ref<string[]>([]);

  async function loadBlocks() {
    const result = await openFile();

    if (result) {
      blocks.value = result;
      usedItemNames.value = items_used_in_blocks(result);
    }
  }

  return {
    blocks,
    usedItemNames,
    loadBlocks,
  };
}
