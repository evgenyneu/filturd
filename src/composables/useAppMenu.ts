import { Menu } from "@tauri-apps/api/menu";
import { onMounted } from "vue";

export function useAppMenu() {
  async function initializeMenu() {
    const menu = await Menu.new({
      items: [
        {
          id: "quit",
          text: "Quit",
          action: () => {
            console.log("quit pressed");
          },
        },
      ],
    });

    await menu.setAsAppMenu();
    console.log("-------------menu loaded-------------");
  }

  onMounted(() => {
    initializeMenu();
  });
}
