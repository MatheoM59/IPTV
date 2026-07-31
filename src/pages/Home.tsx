import { Catalogue } from "../components/Catalogue";
import { Link } from "react-router-dom";
import { catalogs } from "../catalogue";

export const Home = () => {
  return (
    <div className=" flex-col items-center">
      <h1>Catalogue</h1>
      <div className="flex align-items gap-6">
        {catalogs.map((catalog) => (
          <Link key={catalog.id} to={`/app/${catalog.id}`}>
            <Catalogue label={catalog.label} Icon={catalog.Icon} />
          </Link>
        ))}
      </div>
    </div>
  );
};
