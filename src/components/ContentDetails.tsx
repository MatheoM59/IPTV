import { X, Play, TvMinimalPlay } from "lucide-react";
import type { Content } from "../types";
export const ContentDetails = ({
  setOpen,
  open,
}: {
  setOpen: React.Dispatch<React.SetStateAction<Content | undefined>>;
  open: Content;
}) => {
  return (
    <div className="flex flex-col z-5 fixed  inset-10 border border-line rounded-panel p-2 bg-surface">
      <div className="h-1/3 flex flex-col">
        <button
          onClick={() => setOpen(undefined)}
          className="bg-black border border-line rounded-full self-end"
        >
          <X width={20} height={20} />
        </button>
        <h2 className="mt-auto ml-4 font-display capitalize text-title font-semibold text-shadow-art">
          {open.title}
        </h2>
      </div>
      <div className="flex-1 min-h-0 overflow-y-auto p-4">
        <div className="flex items-center gap-2 text-caption text-subtle tabular-nums">
          <p>date · durée · Genre · Genre · Genre</p>
          <p>⭐️ Note</p>
          <div className="border border-line rounded-md px-1.5 py-0.5 text-micro font-medium tracking-label uppercase">
            Extension · Quality
          </div>
        </div>
        <div className="flex gap-4 mt-5">
          <button className="flex items-center gap-1 bg-accent p-2 rounded-card text-label font-medium hover:bg-accent-hover">
            <Play /> Lire le film
          </button>
          <a href="https://youtube.com" target="_blank">
            <button className="flex items-center gap-1 border border-line p-2 rounded-card text-label font-medium hover:bg-accent-soft">
              <TvMinimalPlay /> Regarder la bande annonce
            </button>
          </a>
        </div>
        <div className="mt-10 border-b border-line">
          <p className="w-2/5 pb-15 text-body text-muted">
            Chris Kyle, tireur d'élite des Navy SEALs, est envoyé en Irak avec
            une mission : protéger ses frères d'armes. Sa précision sauve
            d'innombrables vies sur le champ de bataille, et sa réputation de
            légende se répand. Mais de retour auprès des siens, il découvre que
            c'est la guerre qu'il n'arrive plus à laisser derrière lui.
          </p>
        </div>
        <div className="flex justify-between pt-5">
          <div className="flex flex-col gap-1">
            <p className="text-micro font-medium tracking-label uppercase text-subtle">
              Réalisation
            </p>
            <p className="text-body">Moi</p>
          </div>
          <div className="flex flex-col gap-1">
            <p className="text-micro font-medium tracking-label uppercase text-subtle">
              Distribution
            </p>
            <p className="text-body"></p>
          </div>
          <div className="flex flex-col gap-1">
            <p className="text-micro font-medium tracking-label uppercase text-subtle">
              Langue
            </p>
            <p className="text-body"></p>
          </div>
          <div className="flex flex-col gap-1">
            <p className="text-micro font-medium tracking-label uppercase text-subtle">
              Ajouté le
            </p>
            <p className="text-body"></p>
          </div>
        </div>
      </div>
    </div>
  );
};
