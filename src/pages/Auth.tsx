import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import { useNavigate } from "react-router-dom";

export const Auth = () => {
  const [host, setHost] = useState("");
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const nav = useNavigate();
  const handleSubmit = async () => {
    try {
      const account = await invoke("get_account_info", {
        host,
        username,
        password,
      });
      nav("/app", { replace: true });
      console.log(account);
    } catch (e) {
      console.error(e);
    }
  };
  const champ =
    "w-full rounded-lg border border-line bg-base px-4 py-2.5 text-ink " +
    "placeholder:text-muted outline-none transition " +
    "focus:border-accent focus:ring-2 focus:ring-accent/30";

  return (
    <main className="flex h-full items-center justify-center p-6">
      <form className="w-full max-w-sm rounded-2xl border border-line bg-surface p-8 shadow-2xl shadow-black/40">
        <h1 className="text-2xl font-semibold tracking-tight">Ton IPTV</h1>
        <p className="mt-1 mb-8 text-sm text-muted">
          Connecte-toi avec les identifiants de ton fournisseur.
        </p>

        <div className="flex flex-col gap-4">
          <label className="flex flex-col gap-1.5">
            <span className="text-xs font-medium text-muted">Identifiant</span>
            <input
              type="text"
              placeholder="Ton username"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              className={champ}
            />
          </label>

          <label className="flex flex-col gap-1.5">
            <span className="text-xs font-medium text-muted">Mot de passe</span>
            <input
              type="password"
              placeholder="Ton password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className={champ}
            />
          </label>

          <label className="flex flex-col gap-1.5">
            <span className="text-xs font-medium text-muted">Serveur</span>
            <input
              type="text"
              placeholder="http://exemple.com"
              value={host}
              onChange={(e) => setHost(e.target.value)}
              className={champ}
            />
          </label>
        </div>

        <button
          type="button"
          onClick={handleSubmit}
          className="mt-8 w-full rounded-lg bg-accent px-4 py-2.5 font-medium text-ink transition hover:bg-accent-hover active:scale-[0.99]"
        >
          Se connecter
        </button>
      </form>
    </main>
  );
};
