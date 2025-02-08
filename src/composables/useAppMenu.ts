import { Menu } from "@tauri-apps/api/menu";
import { onMounted } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";

export function useAppMenu() {
  async function initializeMenu() {
    const menu = await Menu.new({
      items: [
        {
          id: "file",
          text: "File",
          items: [
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
