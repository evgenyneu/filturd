import { Menu, PredefinedMenuItem } from "@tauri-apps/api/menu";
import { onMounted } from "vue";
import { useBlocks } from "./useBlocks";
import { getCurrentWindow } from "@tauri-apps/api/window";

export function useAppMenu() {
  const { loadBlocks } = useBlocks();

  async function initializeMenu() {
    const separator = await PredefinedMenuItem.new({
      item: "Separator",
    });

    const menu = await Menu.new({
      items: [
        {
          id: "file",
          text: "File",
          items: [
            {
              id: "open",
              text: "Open Filter...",
              action: async () => {
                await loadBlocks();
              },
            },
            separator,
            {
              id: "exit",
              text: "Exit",
              action: async () => {
                await getCurrentWindow().close();
              },
            },
          ],
        },
      ],
    });

    await menu.setAsAppMenu();
  }

  onMounted(() => {
    initializeMenu();
  });
}
