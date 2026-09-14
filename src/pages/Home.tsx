import { Catalogue } from "../components/Catalogue";
import { NavLink } from "react-router-dom";
import { catalogs } from "../catalogue";
import { useEffect, useState } from "react";
import type { AccountView } from "../types";
import { invoke } from "@tauri-apps/api/core";
import { formatExpDate } from "../GlobalFunction";

export const Home = () => {
  const [accountInfo, setAccountInfo] = useState<AccountView | null>(null);
  useEffect(() => {
    const charger = async () => {
      try {
        const call = await invoke<AccountView>("get_account");
        setAccountInfo(call);
      } catch (e) {
        console.error("Error : ", { e });
      }
    };
    charger();
  }, []);
  const expDate = formatExpDate(accountInfo?.exp_date);

  return (
    <div
      className="flex flex-col items-center gap-6 min-h-0 overflow-y-auto 
    w-full h-full justify-center p-8 bg-base "
    >
      <div className="flex flex-col items-center gap-4 text-center">
        <h2 className="font-display text-hero font-semibold tracking-title">
          Qu'est-ce qu'on regarde ?
        </h2>
        {expDate && (
          <p className="text-caption text-subtle tabular-nums">
            Abonnement actif jusqu'au {expDate}.
          </p>
        )}
      </div>

      <div className="flex items-center gap-6 w-2/3 ">
        {catalogs.map((catalog) => (
          <NavLink key={catalog.id} to={`/app/${catalog.id}`} className="w-1/3">
            <Catalogue label={catalog.label} Icon={catalog.Icon} />
          </NavLink>
        ))}
      </div>
    </div>
  );
};
