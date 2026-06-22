import { useEffect, useState } from "react";
import { initDatabase } from "./lib/api";
import { PackagesPage } from "./pages/PackagesPage";
import { EntriesPage } from "./pages/EntriesPage";
import { RelationsPage } from "./pages/RelationsPage";
import { ImportPage } from "./pages/ImportPage";

type Page = "packages" | "entries" | "relations" | "import";

export default function App() {
  const [page, setPage] = useState<Page>("packages");
  const [ready, setReady] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    initDatabase()
      .then(() => setReady(true))
      .catch((e) => {
        console.error(e);
        setError(String(e));
      });
  }, []);

  if (error) {
    return (
      <div className="app">
        <h1>ORKB Editor</h1>
        <p className="error">Database error: {error}</p>
      </div>
    );
  }

  if (!ready) {
    return (
      <div className="app">
        <h1>ORKB Editor</h1>
        <p>Initializing database...</p>
      </div>
    );
  }

  return (
    <div className="app">
      <aside className="sidebar">
        <h1>ORKB</h1>

        <button onClick={() => setPage("packages")}>Packages</button>
        <button onClick={() => setPage("entries")}>Entries</button>
        <button onClick={() => setPage("relations")}>Relations</button>
        <button onClick={() => setPage("import")}>Import</button>
      </aside>

      <main className="main">
        {page === "packages" && <PackagesPage />}
        {page === "entries" && <EntriesPage />}
        {page === "relations" && <RelationsPage />}
        {page === "import" && <ImportPage />}
      </main>
    </div>
  );
}
