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

export type Content = {
  id: number;
  title: string;
  image: string | null;
  category_id: string | null;
  extention: string | null;
};
