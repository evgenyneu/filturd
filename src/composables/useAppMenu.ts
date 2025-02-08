import { Menu, PredefinedMenuItem } from "@tauri-apps/api/menu";
import { onMounted } from "vue";
import { openFile } from "../utils/fileOpener";

export function useAppMenu() {
  async function initializeMenu() {
    const separator = await PredefinedMenuItem.new({
      item: "Separator",
    });

    const quit = await PredefinedMenuItem.new({
      item: "CloseWindow",
      text: "Exit",
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
              action: () => {
                openFile();
              },
            },
            separator,
            quit,
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
