import { useParams } from "react-router-dom";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { SideCategory } from "../components/SideCategory";
import type { Category, Content } from "../types";
import { ContentDisplay } from "../components/Content";
export const Browse = () => {
  const [category, setCategory] = useState<string | null>(null);
  const [categoryList, setCategoryList] = useState<Category[]>([]);
  const [content, setContent] = useState<Content[]>([]);
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

  useEffect(() => {
    const charger = async () => {
      try {
        const call = await invoke<Content[]>("get_contents", {
          catalog,
          categoryId: category,
        });
        setContent(call);
        console.log("Succes content");
      } catch (e) {
        console.error("Erreur lors du chargement de contents : (", { e }, ")");
      }
    };
    charger();
  }, [catalog, category]);
  return (
    <div className="flex flex-1 min-h-0">
      <SideCategory setCategory={setCategory} categoryList={categoryList} />
      <ContentDisplay category={category} content={content} />
    </div>
  );
};
