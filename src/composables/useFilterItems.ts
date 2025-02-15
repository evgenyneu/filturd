import { ref } from "vue";
import type { FilterItems } from "../types/items";
import items from "../../src-tauri/src/filter/items.json";

const filterItems = ref<FilterItems>(items);

export function useFilterItems() {
  return {
    items: filterItems,
    getItem: (name: string) => filterItems.value[name],
    isCondition: (name: string) =>
      filterItems.value[name]?.is_condition ?? false,
    isAction: (name: string) => !filterItems.value[name]?.is_condition ?? false,
  };
}
