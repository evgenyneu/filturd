import type { Items } from "../types/items";
import type { Block } from "../../src-tauri/bindings/Block";
import itemsData from "./items.json";

export const items: Items = itemsData;

export function items_used_in_blocks(blocks: Block[]): string[] {
  const uniqueItems = new Set<string>();

  blocks.forEach((block) => {
    block.items.forEach((item) => {
      uniqueItems.add(item.name);
    });
  });

  return Array.from(uniqueItems);
}
