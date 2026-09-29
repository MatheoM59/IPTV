import { Play } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
export const BtnPlay = ({
  catalog,
  titre,
  id,
  extension,
}: {
  catalog: string;
  titre: string;
  id: number;
  extension: string | undefined;
}) => {
  const handleClick = async () => {
    try {
      await invoke("lire", { catalog, titre, id, extension });
    } catch (e) {
      console.error("Error : ", { e });
    }
    console.log("btn clicked");
  };
  return (
    <button
      className="flex items-center gap-1 bg-accent p-2 
    rounded-card text-label font-medium hover:bg-accent-hover"
      onClick={handleClick}
    >
      <Play /> Lire le film
    </button>
  );
};
