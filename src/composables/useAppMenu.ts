import { Menu } from "@tauri-apps/api/menu";
import { onMounted } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { openFile } from "../utils/fileOpener";

export function useAppMenu() {
  async function initializeMenu() {
    const menu = await Menu.new({
      items: [
        {
          id: "file",
          text: "File",
          items: [
            {
              id: "open",
              text: "Open Filter",
              action: () => {
                openFile();
              },
            },
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
