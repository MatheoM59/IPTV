import { ModaleTitle } from "./ModaleTitle";
import type { Content } from "../types";
import { BtnPlay } from "./BtnPlay";
import { BtnTrailer } from "./BtnTrailer";
export const SerieDetails = ({ open }: { open: Content }) => {
  return (
    <div>
      <ModaleTitle src={open.image} blur />
      <div className="flex-1 min-h-0 overflow-y-auto p-4">
        {/* MetaData attend la commande Rust des series : pas encore de donnees */}
        <div className="flex gap-4 mt-5">
          <BtnPlay />
          <BtnTrailer url={"https://youtube.com"} />
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
