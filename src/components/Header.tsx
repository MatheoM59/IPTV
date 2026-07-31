import { Link } from "react-router-dom";
import { catalogs } from "../catalogue";
export const Header = () => {
  return (
    <div className="flex items-center gap-6 px-6 py-4 justify-between border-b border-gray/10">
      <h1>Ton IPTV</h1>
      <nav className="flex gap-4">
        {catalogs.map((catalog) => (
          <Link key={catalog.id} to={`/app/${catalog.id}`}>
            <catalog.Icon />
          </Link>
        ))}
      </nav>
    </div>
  );
};
