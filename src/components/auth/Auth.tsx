import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
export const Auth = () => {
  const [host, setHost] = useState("");
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const handleSubmit = async () => {
    try {
      const account = await invoke("get_account_info", {
        host,
        username,
        password,
      });
      console.log(account);
    } catch (e) {
      console.error(e);
    }
  };
  return (
    <>
      <form>
        <div className="logins">
          <input
            type="text"
            placeholder="Ton username"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
          />
          <input
            type="password"
            placeholder="Ton password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
          />
        </div>
        <div>
          <input
            type="text"
            placeholder="Ton liens de connection"
            value={host}
            onChange={(e) => setHost(e.target.value)}
          />
        </div>
        <button type="button" onClick={handleSubmit}>
          Se connecter
        </button>
      </form>
    </>
  );
};
