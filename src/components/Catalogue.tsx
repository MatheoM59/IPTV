import { CatalogEntry } from "../types";
export const Catalogue = ({
  label,
  Icon,
}: Pick<CatalogEntry, "label" | "Icon">) => {
  return (
    <div
      className="flex flex-col gap-6  border border-line rounded-card
     bg-surface aspect-[1/1] p-6 hover:bg-elevated group hover:border-accent"
    >
      <Icon className="size-12 text-muted group-hover:text-accent-hover" />
      <h2 className="font-display text-title font-semibold">{label}</h2>
      <p className="mt-auto group-hover:text-accent-hover">Entrer -{`>`} </p>
    </div>
  );
};
