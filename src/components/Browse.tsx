import { useEffect, useState } from "react";
import { useSearchParams } from "react-router-dom";
import { invoke } from "@tauri-apps/api/core";
import { SideCategory } from "./SideCategory";
import { ContentDisplay } from "./Content";
import { Loading } from "./Loading";
import { Error } from "./Error";
import type { Category, Content } from "../types";

export const Browse = ({ catalog }: { catalog: string }) => {
  const [category, setCategory] = useState<string | null>(null);
  const [categoryList, setCategoryList] = useState<Category[]>([]);
  const [searchParams, setSearchParams] = useSearchParams();
  const q = searchParams.get("q");
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
      } catch (e) {
        setError(true);
      }
    };
    charger();
  }, [catalog]);

  useEffect(() => {
    if (q) {
      const charger = async () => {
        setLoading(true);
        try {
          const call = await invoke<Content[]>("search_content", {
            catalog,
            query: q,
          });
          setContent(call);
        } catch (e) {
          setError(true);
        } finally {
          setLoading(false);
        }
      };
      charger();
    } else if (category !== null) {
      const charger = async () => {
        setLoading(true);
        try {
          const call = await invoke<Content[]>("get_content", {
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
    }
  }, [catalog, category, q]);

  const categorySelected = categoryList.find(
    (c) => c.category_id === category,
  )?.category_name;

  const categorySearch = (c: string | null) => {
    setCategory(c);
    const url = new URLSearchParams(searchParams);
    url.delete("q");
    setSearchParams(url, { replace: true });
  };

  return (
    <div className="flex flex-1 min-h-0 shrink-0 overflow-y-hidden ">
      <SideCategory
        setCategory={categorySearch}
        categorySelected={q ? null : category}
        categoryList={categoryList}
        catalog={catalog}
      />
      {!loading && (
        <ContentDisplay
          categorySelected={q ? "Résultat pour " + q : categorySelected}
          content={content}
          catalog={catalog}
        />
      )}
      {loading && <Loading />}
      {error && <Error />}
    </div>
  );
};
