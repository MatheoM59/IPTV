import { useParams } from "react-router-dom";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { SideCategory } from "../components/SideCategory";
import type { Category } from "../types";
export const Browse = () => {
  const [category, setCategory] = useState("");
  const [categoryList, setCategoryList] = useState<Category[]>([]);
  const { catalog } = useParams();

  useEffect(() => {
    const charger = async () => {
      try {
        const call = await invoke<Category[]>("get_categories", {
          catalog,
        });
        setCategoryList(call);
        if (catalog === "live") {
          setCategory("5");
        }
        if (catalog === "vod") {
          setCategory("654");
        }
        if (catalog === "serie") {
          setCategory("164");
        }
        console.log("try");
      } catch (e) {
        console.error(e);
      }
    };
    charger();
  }, [catalog]);
  return (
    <div className="flex flex-1 min-h-0">
      <SideCategory setCategory={setCategory} categoryList={categoryList} />
      <h3>{category}</h3>
    </div>
  );
};
