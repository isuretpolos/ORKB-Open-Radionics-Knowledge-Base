// src/pages/RelationsPage.tsx

import { useEffect, useState } from "react";
import { DbRelation, listRelations } from "../lib/api";

export function RelationsPage() {
  const [relations, setRelations] = useState<DbRelation[]>([]);
  const [error, setError] = useState<string | null>(null);

  async function load() {
    try {
      setError(null);
      setRelations(await listRelations());
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
        <h2>Relations</h2>
        <button onClick={load}>Reload</button>
      </div>

      {error && <p className="error">{error}</p>}

      <table>
        <thead>
          <tr>
            <th>Relation</th>
            <th>From</th>
            <th>To</th>
            <th>Weight</th>
            <th>Package</th>
          </tr>
        </thead>
        <tbody>
          {relations.map((r) => (
            <tr key={r.id}>
              <td>{r.relation_type}</td>
              <td className="mono">{r.from_entry_id}</td>
              <td className="mono">{r.to_entry_id}</td>
              <td>{r.weight}</td>
              <td className="mono">{r.package_id}</td>
            </tr>
          ))}
        </tbody>
      </table>

      {relations.length === 0 && <p>No relations imported yet.</p>}
    </section>
  );
}
