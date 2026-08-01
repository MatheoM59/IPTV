import type { Category } from "../types";
export const SideCategory = ({
  setCategory,
  categoryList,
}: {
  setCategory: (id: string) => void;
  categoryList: Category[];
}) => {
  return (
    <div className="w-1/5 shrink-0 overflow-y-auto border-r border-gray-300">
      <h2>Catégories</h2>
      <div>
        {categoryList.map((category) => (
          <div
            key={category.category_id}
            onClick={() => setCategory(category.category_id)}
            className="border-b border-gray-300"
          >
            <h3>{category.category_name}</h3>
          </div>
        ))}
      </div>
    </div>
  );
};
