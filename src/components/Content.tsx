import type { Content } from "../types";
import { ContentCard } from "./ContentCard";
export const ContentDisplay = ({
  category,
  content,
}: {
  category: string | null;
  content: Content[];
}) => {
  return (
    <div className=" min-h-0 w-full ">
      <h1 className="text-center border-b border-line w-full mb-6">
        {category}
      </h1>
      <div className="grid grid-cols-[repeat(auto-fill,minmax(14rem,1fr))] gap-6 px-6 pb-6">
        {content.map((item) => (
          <ContentCard item={item} key={item.id} />
        ))}
      </div>
    </div>
  );
};
