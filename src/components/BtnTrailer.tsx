import { openUrl } from "@tauri-apps/plugin-opener";
import { TvMinimalPlay } from "lucide-react";
export const BtnTrailer = ({ url }: { url: string }) => {
  return (
    <div>
      <button
        onClick={async () => openUrl("https://www.youtube.com/watch?v=" + url)}
        className="flex items-center gap-1 border border-line p-2 rounded-card text-label font-medium hover:bg-accent-soft"
      >
        <TvMinimalPlay /> Regarder la vidéo
      </button>
    </div>
  );
};
