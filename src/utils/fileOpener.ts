import { documentDir, join } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import type { Block } from "../../src-tauri/bindings/Block";

function printItemNameFrequencies(blocks: Block[]) {
  const itemCounts = new Map<string, number>();

  blocks.forEach((block) => {
    // Get unique item names in this block
    const uniqueNames = new Set(block.items.map((item) => item.name));

    // Add to total counts
    uniqueNames.forEach((name) => {
      itemCounts.set(name, (itemCounts.get(name) || 0) + 1);
    });
  });

  // Convert to array, sort by frequency descending
  const sortedCounts = Array.from(itemCounts.entries()).sort(
    ([, a], [, b]) => b - a
  );

  // Print in CSV format
  console.log("Item Name, Frequency");
  sortedCounts.forEach(([name, count]) => {
    console.log(`${name}, ${count}`);
  });
}

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

  printItemNameFrequencies(blocks);

  return blocks;
}
