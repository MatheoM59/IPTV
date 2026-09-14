import { Play } from "lucide-react";
export const BtnPlay = () => {
  return (
    <button className="flex items-center gap-1 bg-accent p-2 rounded-card text-label font-medium hover:bg-accent-hover">
      <Play /> Lire le film
    </button>
  );
};
