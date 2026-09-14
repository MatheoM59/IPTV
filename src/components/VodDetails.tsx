import type { Content, MovieDetails } from "../types";
import { invoke } from "@tauri-apps/api/core";
import { BtnTrailer } from "./BtnTrailer";
import { BtnPlay } from "./BtnPlay";
import { MetaData } from "./MetaData";
import { ModaleTitle } from "./ModaleTitle";
import { useEffect, useState } from "react";

export const VodDetails = ({ open }: { open: Content }) => {
  const vodId = open.id.toString();
  const [info, setInfo] = useState<MovieDetails>();

  useEffect(() => {
    const charger = async () => {
      try {
        const call = await invoke<MovieDetails>("get_vod_details", {
          vodId: vodId,
        });
        setInfo(call);
      } catch (e) {
        console.error("Error : ", { e });
      }
    };
    charger();
  }, [vodId]);

  const backdrop = info?.backdrop_path ?? null;

  return (
    <>
      <ModaleTitle src={backdrop ?? open.image} blur={!backdrop} />

      <div className="relative min-h-0 flex-1 overflow-y-auto">
        <div className="flex max-w-2xl flex-col gap-5 p-8">
          <h1 className="font-display text-title font-semibold text-shadow-art">
            {info?.name ?? open.title}
          </h1>

          {info && <MetaData info={info} />}

          <div className="flex gap-3">
            <BtnPlay />
            {info?.youtube_trailer && <BtnTrailer url={info.youtube_trailer} />}
          </div>

          {info?.plot && (
            <p className="text-body text-muted">{info.plot}</p>
          )}

          {info && (
            <dl className="flex flex-wrap gap-x-10 gap-y-4 border-t border-line pt-5">
              {info.director && (
                <div className="flex flex-col gap-1">
                  <dt className="text-micro font-medium tracking-label uppercase text-subtle">
                    Réalisation
                  </dt>
                  <dd className="text-body">{info.director}</dd>
                </div>
              )}
              {info.country && (
                <div className="flex flex-col gap-1">
                  <dt className="text-micro font-medium tracking-label uppercase text-subtle">
                    Pays
                  </dt>
                  <dd className="text-body">{info.country}</dd>
                </div>
              )}
              {info.age && (
                <div className="flex flex-col gap-1">
                  <dt className="text-micro font-medium tracking-label uppercase text-subtle">
                    Limite d'âge
                  </dt>
                  <dd className="text-body">{info.age}</dd>
                </div>
              )}
              {info.cast && (
                <div className="flex w-full flex-col gap-1">
                  <dt className="text-micro font-medium tracking-label uppercase text-subtle">
                    Distribution
                  </dt>
                  <dd className="text-body">{info.cast}</dd>
                </div>
              )}
            </dl>
          )}
        </div>
      </div>
    </>
  );
};
