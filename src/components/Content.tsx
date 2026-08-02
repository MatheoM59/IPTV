import type { Content } from "../types";
export const ContentDisplay = ({
  category,
  content,
}: {
  category: string | null;
  content: Content[];
}) => {
  return (
    <div>
      <h1>{category}</h1>
      {content.map((item) => (
        <div key={item.id}>
          <h2>{item.title}</h2>
        </div>
      ))}
    </div>
  );
};
