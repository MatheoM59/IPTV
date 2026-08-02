import { Browse } from "../components/Browse";
import { useParams } from "react-router-dom";
export const BrowseRoute = () => {
  const { catalog } = useParams();
  if (catalog === undefined) return;
  return <Browse catalog={catalog} key={catalog} />;
};
