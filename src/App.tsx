import "./App.css";
import { Auth } from "./pages/Auth";
import { Layout } from "./components/Layout";
import { HashRouter, Routes, Route } from "react-router-dom";
import { Home } from "./pages/Home";
import { BrowseRoute } from "./pages/BrowseRoute";
function App() {
  return (
    <HashRouter>
      <Routes>
        <Route path="/" element={<Auth />} />
        <Route path="/app" element={<Layout />}>
          <Route index element={<Home />} />
          <Route path=":catalog" element={<BrowseRoute />} />
        </Route>
      </Routes>
    </HashRouter>
  );
}

export default App;
