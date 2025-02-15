import { documentDir, join } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import type { Block } from "../../src-tauri/bindings/Block";

export async function openFile(): Promise<Block[] | null> {
  const documentsDir = await documentDir();
  const poe2Dir = await join(documentsDir, "My Games", "Path of Exile 2");

  const path = await open({
    multiple: false,
    directory: false,
    defaultPath: poe2Dir,
    filters: [
      {
        name: "Filter",
        extensions: ["filter"],
      },
      {
        name: "All files",
        extensions: ["*"],
      },
    ],
  });

  if (!path) return null;
  const blocks = await invoke<Block[]>("open_file", { path });

  return blocks;
}
