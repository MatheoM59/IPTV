import { useParams } from "react-router-dom";
import { useState } from "react";
import { SideCategory } from "../components/SideCategory";
export const Browse = () => {
  const [category, setCategory] = useState("");
  const { catalog } = useParams();
  return (
    <div>
      <h1>{catalog}</h1>
      <SideCategory setCategory={setCategory} />
    </div>
  );
};
