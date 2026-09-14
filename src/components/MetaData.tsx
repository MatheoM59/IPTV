import { MovieDetails } from "../types";

export const MetaData = ({ info }: { info: MovieDetails }) => {
  return (
    <div className="flex items-center gap-3 text-caption text-subtle tabular-nums">
      <p>{info.releasedate}</p>·<p>{info.duration}</p>·<p>{info.genre}</p>
      {info.rating && <p>⭐️ {info.rating}</p>}
      <div className="border border-line rounded-md px-1.5 py-0.5 text-micro font-medium tracking-label uppercase">
        {info.container_extension} · {info.quality}
      </div>
    </div>
  );
};
