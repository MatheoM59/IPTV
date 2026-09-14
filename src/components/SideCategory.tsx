import { SearchBar } from "./Search";
import { catalogs } from "../catalogue";
import { useState } from "react";
import { formatSearch } from "../GlobalFunction";
import type { Category } from "../types";

export const SideCategory = ({
  setCategory,
  categoryList,
  categorySelected,
  catalog,
}: {
  setCategory: (id: string | null) => void;
  categorySelected: string | null;
  categoryList: Category[];
  catalog: string;
}) => {
  const classCommune = "flex items-center p-2 rounded-panel ";
  const catalogSelected = catalogs.find((c) => c.id === catalog)?.label;
  const [search, setSearch] = useState("");
  const searchFormated = formatSearch(search);
  const list = categoryList.filter((category) =>
    formatSearch(category.category_name).includes(searchFormated),
  );
  return (
    <div className="w-1/5 shrink-0 overflow-y-auto border-r flex flex-col border-line bg-surface ">
      <div className=" border-b border-line p-3 w-full">
        <h2 className="uppercase text-subtle text-caption">
          Catégories · {catalogSelected}
        </h2>
        <SearchBar
          search={search}
          setSearch={setSearch}
          placeHolder="Filtrer..."
          prefix={false}
        />
      </div>
      <div className="p-3">
        {list.length === 0 && (
          <div>
            <h3 className="font-bold">Aucune catégorie correspondante</h3>
          </div>
        )}
        {list.map((category) => (
          <div
            key={category.category_id}
            onClick={() => setCategory(category.category_id)}
            className={`${classCommune} ${
              category.category_id === categorySelected
                ? "bg-accent-soft border-l-3 border-accent text-ink"
                : "text-muted hover:bg-elevated hover:text-ink hover:border-l-3 border-ink"
            }`}
          >
            <h3 className="capitalize font-bold">
              {category.category_name.toLowerCase()}
            </h3>
          </div>
        ))}
      </div>
    </div>
  );
};
