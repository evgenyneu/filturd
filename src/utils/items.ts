import type { Items } from "../types/items";
import type { Block } from "../../src-tauri/bindings/Block";
import itemsData from "./items.json";

export const items: Items = itemsData;

export function items_used_in_blocks(blocks: Block[]): string[] {
  return Array.from(
    new Set(blocks.flatMap((block) => Object.keys(block.items)))
  );
}
