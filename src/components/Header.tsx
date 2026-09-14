import { Link } from "react-router-dom";
import { catalogs } from "../catalogue";
import { User } from "lucide-react";
import { useParams, useSearchParams } from "react-router-dom";
import { SearchBar } from "./Search";
export const Header = () => {
  const { catalog } = useParams();
  const catalogSelected = catalog;
  const [searchParams, setSearchParams] = useSearchParams();
  const search = searchParams.get("q") ?? "";
  const SetSearch = (value: string) => {
    const params = new URLSearchParams(searchParams);
    if (value === "") {
      params.delete("q");
    } else {
      params.set("q", value);
    }
    setSearchParams(params, { replace: true });
  };

  const catalogActif = catalogs.find((c) => c.id === catalogSelected);
  return (
    <div className="flex items-stretch gap-6 px-6 justify-between border-b border-line-strong bg-surface h-20">
      <h1 className="text-ink flex items-center">Ton IPTV</h1>
      <nav className="flex gap-6 text-body ">
        {catalogs.map((catalog) => (
          <Link
            key={catalog.id}
            to={`/app/${catalog.id}`}
            className={` flex h-full items-center ${catalog.id === catalogSelected ? "  border-b-3 border-accent " : ""}`}
          >
            <div className="group flex gap-3">
              <catalog.Icon
                className={` group-hover:text-accent-hover  ${catalog.id === catalogSelected ? "text-accent" : "text-muted"}`}
              />
              <h2
                className={` group-hover:text-ink  ${catalog.id === catalogSelected ? "text-ink" : "text-muted "}`}
              >
                {catalog.label}
              </h2>
            </div>
          </Link>
        ))}
      </nav>
      {catalogActif && (
        <SearchBar
          search={search}
          setSearch={SetSearch}
          placeHolder="Rechercher un titre..."
          className="max-w-104 min-w-52 flex-1 self-center"
          prefix={
            <span
              className="flex shrink-0 items-center gap-1 rounded-md bg-accent-soft px-1.5 py-0.5
                text-micro font-semibold tracking-label text-accent-hover"
            >
              <catalogActif.Icon className="size-3" />
              {catalogActif.label}
            </span>
          }
        />
      )}
      <User className="flex self-center border border-accent text-accent hover:text-accent-hover hover:border-accent-hover rounded-panel p-1" />
    </div>
  );
};
