import { ContentCard } from "./ContentCard";
import { useState } from "react";
import type { Content } from "../types";
import { Modale } from "./Modale";
import { VodDetails } from "./VodDetails";
import { SerieDetails } from "./SerieDetails";
export const ContentDisplay = ({
  categorySelected,
  content,
  catalog,
}: {
  categorySelected?: string;
  content: Content[];
  catalog: string;
}) => {
  const [open, setOpen] = useState<Content>();
  const handleClose = () => {
    setOpen(undefined);
  };
  return (
    <div className=" min-h-0 w-full h-screen  overflow-y-auto ">
      <div className="h-1/6 flex items-center p-6 border-b border-line mb-4 gap-3">
        <h2 className="text-title font-serif">{categorySelected}</h2>
        <p className="text-subtle tabular-nums">
          {content.length} {content.length > 1 ? "titres" : "titre"}
        </p>
      </div>
      <div className="grid grid-cols-[repeat(auto-fill,minmax(14rem,1fr))] gap-6 px-6 pb-6">
        {content.length > 0 ? (
          content.map((item) => (
            <div onClick={() => setOpen(item)} key={item.id}>
              <ContentCard item={item} />
            </div>
          ))
        ) : (
          <h1>Aucun contenue de disponible </h1>
        )}
      </div>

      {open != undefined && (
        <Modale onClose={handleClose}>
          {catalog === "vod" && <VodDetails open={open} />}
          {catalog === "serie" && <SerieDetails open={open} />}
        </Modale>
      )}
    </div>
  );
};
