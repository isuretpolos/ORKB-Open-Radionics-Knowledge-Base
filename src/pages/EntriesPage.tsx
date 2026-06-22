import { useEffect, useState } from "react";
import { DbEntry, listEntries, searchEntries } from "../lib/api";

export function EntriesPage() {
  const [entries, setEntries] = useState<DbEntry[]>([]);
  const [query, setQuery] = useState("");
  const [selected, setSelected] = useState<DbEntry | null>(null);
  const [error, setError] = useState<string | null>(null);

  async function loadAll() {
    try {
      setError(null);
      setEntries(await listEntries());
    } catch (e) {
      console.error(e);
      setError(String(e));
    }
  }

  async function search() {
    try {
      setError(null);

      if (!query.trim()) {
        await loadAll();
        return;
      }

      setEntries(await searchEntries(query));
    } catch (e) {
      console.error(e);
      setError(String(e));
    }
  }

  useEffect(() => {
    loadAll();
  }, []);

  return (
    <section>
      <div className="pageHeader">
        <h2>Entries</h2>
        <button onClick={loadAll}>Reload</button>
      </div>

      <div className="toolbar">
        <input
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Search entries..."
          onKeyDown={(e) => {
            if (e.key === "Enter") search();
          }}
        />
        <button onClick={search}>Search</button>
      </div>

      {error && <p className="error">{error}</p>}

      <div className="split">
        <table>
          <thead>
            <tr>
              <th>Type</th>
              <th>Key</th>
              <th>Name</th>
              <th>Package</th>
            </tr>
          </thead>
          <tbody>
            {entries.map((e) => (
              <tr
                key={e.id}
                onClick={() => setSelected(e)}
                className={selected?.id === e.id ? "selected" : ""}
              >
                <td>{e.entry_type}</td>
                <td className="mono">{e.key}</td>
                <td>{e.name}</td>
                <td className="mono">{e.package_id}</td>
              </tr>
            ))}
          </tbody>
        </table>

        <aside className="details">
          {selected ? (
            <>
              <h3>{selected.name}</h3>
              <p className="mono">{selected.id}</p>
              <p>{selected.description}</p>

              <h4>Data</h4>
              <pre>{formatJson(selected.data_json)}</pre>

              <h4>Tags</h4>
              <pre>{formatJson(selected.tags_json)}</pre>
            </>
          ) : (
            <p>Select an entry.</p>
          )}
        </aside>
      </div>
    </section>
  );
}

function formatJson(value?: string | null): string {
  if (!value) return "";

  try {
    return JSON.stringify(JSON.parse(value), null, 2);
  } catch {
    return value;
  }
}
