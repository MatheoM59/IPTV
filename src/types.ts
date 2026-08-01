import { LucideIcon } from "lucide-react";
export type Catalog = "live" | "vod" | "serie";
export type CatalogEntry = {
  id: Catalog;
  label: string;
  Icon: LucideIcon;
};

export type Category = {
  category_id: string;
  category_name: string;
  parent_id: number;
};
