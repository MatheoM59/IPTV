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

export type AccountView = {
  host: string;
  username: string;
  status: string;
  exp_date: string | null;
};

export type MovieDetails = {
  name: string;
  duration: string;
  container_extension: string;
  stream_id: number;
  rating: number | null;
  plot: string | null;
  cast: string | null;
  director: string | null;
  genre: string | null;
  releasedate: string | null;
  age: string | null;
  country: string | null;
  backdrop_path: string | null;
  youtube_trailer: string | null;
  quality: string | null;
};
