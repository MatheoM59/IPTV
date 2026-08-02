import type { Category } from "../types";
export const SideCategory = ({
  setCategory,
  categoryList,
}: {
  setCategory: (id: string | null) => void;
  categoryList: Category[];
}) => {
  return (
    <div className="w-1/5 shrink-0 overflow-y-auto border-r border-line bg-surface">
      <h2>Catégories</h2>
      <div>
        {categoryList.map((category) => (
          <div
            key={category.category_id}
            onClick={() => setCategory(category.category_id)}
            className="border-b border-line-strong"
          >
            <h3 className="text-">{category.category_name}</h3>
          </div>
        ))}
      </div>
    </div>
  );
};
