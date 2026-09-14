import { Search, X } from "lucide-react";
export const SearchBar = ({
  search,
  setSearch,
  placeHolder,
  prefix,
  className = "",
}: {
  search: string;
  setSearch: (name: string) => void;
  placeHolder: string;
  prefix?: React.ReactNode;
  className?: string;
}) => {
  const handleDelete = () => {
    setSearch("");
  };
  return (
    <div
      className={`group flex h-9 items-center gap-2 rounded-card border border-line bg-base px-3
        transition-colors hover:border-line-strong
        focus-within:border-accent focus-within:ring-2 focus-within:ring-accent/30 ${className}`}
    >
      <Search className="size-4 shrink-0 text-subtle transition-colors group-focus-within:text-accent" />
      {prefix}
      <input
        type="text"
        placeholder={placeHolder}
        className="min-w-0 flex-1 border-0 bg-transparent text-label text-ink outline-none
          placeholder:text-subtle focus:placeholder:text-muted"
        value={search}
        onChange={(e) => {
          setSearch(e.target.value);
        }}
      />
      {search !== "" && (
        <button
          type="button"
          aria-label="Effacer la recherche"
          className="grid size-4 shrink-0 place-items-center rounded-full text-subtle
            transition-colors hover:text-ink focus-visible:outline-2 focus-visible:outline-accent"
          onClick={handleDelete}
        >
          <X className="size-3.5" />
        </button>
      )}
    </div>
  );
};
