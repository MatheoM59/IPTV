import { Clapperboard, Tv, Film } from "lucide-react";
import { CatalogEntry } from "./types";
export const catalogs: CatalogEntry[] = [
  { id: "live", label: "Live", Icon: Tv },
  {
    id: "vod",
    label: "Film",
    Icon: Film,
  },
  {
    id: "serie",
    label: "Série",
    Icon: Clapperboard,
  },
];
