import { LucideIcon } from "lucide-react";
export type Catalog = "live" | "vod" | "serie";
export type CatalogEntry = {
  id: Catalog;
  label: string;
  Icon: LucideIcon;
};
