export interface Item {
  is_condition: boolean;
  frequency: number;
}

export interface Items {
  [key: string]: Item;
}
