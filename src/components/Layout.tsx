import { Outlet } from "react-router-dom";
import { Header } from "./Header";

export const Layout = () => {
  return (
    <div className="h-screen flex flex-col">
      <Header />
      <main className="flex flex-1 min-h-0">
        <Outlet />
      </main>
    </div>
  );
};
