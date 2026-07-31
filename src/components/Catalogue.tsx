import { CatalogEntry } from "../types";
export const Catalogue = ({
  label,
  Icon,
}: Pick<CatalogEntry, "label" | "Icon">) => {
  return (
    <div className="border px-6 py-6">
      <Icon />
      <h2>{label}</h2>
    </div>
  );
};
