import { useEffect, useState } from "react";
import { DbPackage, listPackages } from "../lib/api";

export function PackagesPage() {
  const [packages, setPackages] = useState<DbPackage[]>([]);
  const [error, setError] = useState<string | null>(null);

  async function load() {
    try {
      setError(null);
      setPackages(await listPackages());
    } catch (e) {
      console.error(e);
      setError(String(e));
    }
  }

  useEffect(() => {
    load();
  }, []);

  return (
    <section>
      <div className="pageHeader">
        <h2>Packages</h2>
        <button onClick={load}>Reload</button>
      </div>

      {error && <p className="error">{error}</p>}

      <table>
        <thead>
          <tr>
            <th>Active</th>
            <th>Package ID</th>
            <th>Name</th>
            <th>Version</th>
            <th>Author</th>
            <th>License</th>
            <th>Imported</th>
          </tr>
        </thead>
        <tbody>
          {packages.map((p) => (
            <tr key={p.id}>
              <td>{p.active ? "yes" : "no"}</td>
              <td className="mono">{p.id}</td>
              <td>{p.name}</td>
              <td>{p.version}</td>
              <td>{p.author_name}</td>
              <td>{p.license ?? ""}</td>
              <td>{p.imported_at}</td>
            </tr>
          ))}
        </tbody>
      </table>

      {packages.length === 0 && <p>No packages imported yet.</p>}
    </section>
  );
}
