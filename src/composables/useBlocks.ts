import { ref } from "vue";
import { openFile } from "../utils/fileOpener";

export function useBlocks() {
    const blocks = ref<Array<{
        order: number;
        name: string;
        items: Array<{ name: string; params: string[] }>;
    }>>([]);

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
