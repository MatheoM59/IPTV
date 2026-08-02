import type { Content } from "../types";
export const ContentCard = ({ item }: { item: Content }) => {
  return (
    <div className="  bg-surface border border-line rounded-card">
      <div>
        {item.image && (
          <img
            src={item.image}
            alt={item.title}
            className=" object-cover aspect-[2/3] rounded-card  w-full h-full"
          ></img>
        )}
      </div>
      <p className="border-t border-line text-muted text-center line-clamp-2 ">
        {item.title}
      </p>
    </div>
  );
};
