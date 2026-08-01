import { Outlet } from "react-router-dom";
import { Header } from "./Header";

export const Layout = () => {
  return (
    <div className="h-screen flex flex-col">
      <Header />
      <Outlet />
    </div>
  );
};
