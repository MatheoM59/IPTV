import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { SideCategory } from "./SideCategory";
import type { Category, Content } from "../types";
import { ContentDisplay } from "./Content";
import { Loading } from "./Loading";
export const Browse = ({ catalog }: { catalog: string }) => {
  const [category, setCategory] = useState<string | null>(null);
  const [categoryList, setCategoryList] = useState<Category[]>([]);
  const [content, setContent] = useState<Content[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(false);

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
        setError(true);
      }
    };
    charger();
  }, [catalog]);

  useEffect(() => {
    if (category === null) return;
    const charger = async () => {
      setLoading(true);
      try {
        const call = await invoke<Content[]>("get_contents", {
          catalog,
          categoryId: category,
        });
        setContent(call);
        console.log("Succes content");
      } catch (e) {
        setError(true);
      } finally {
        setLoading(false);
      }
    };
    charger();
  }, [catalog, category]);
  return (
    <div className="flex flex-1 min-h-0">
      <SideCategory setCategory={setCategory} categoryList={categoryList} />
      <ContentDisplay category={category} content={content} />
      {loading && <Loading />}
    </div>
  );
};
