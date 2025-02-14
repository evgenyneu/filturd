import { ref } from "vue";
import { openFile } from "../utils/fileOpener";
import type { Block } from "../../src-tauri/bindings/Block";

export function useBlocks() {
    const blocks = ref<Block[]>([]);

    async function loadBlocks() {
        const result = await openFile();

        if (result) {
            blocks.value = result;
        }
    }

    return {
        blocks,
        loadBlocks
    }
}
