import { useState } from "react";
import { importPackage } from "../lib/api";

export function ImportPage() {
  const [path, setPath] = useState("packages/isuret.core.levels");
  const [message, setMessage] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  async function runImport() {
    try {
      setError(null);
      setMessage(null);

      await importPackage(path);

      setMessage(`Imported: ${path}`);
    } catch (e) {
      console.error(e);
      setError(String(e));
    }
  }

  return (
    <section>
      <h2>Import Package</h2>

      <p>
        First version imports an ORKB package from a local directory containing
        manifest.json, entries.json, relations.json and profiles.json.
      </p>

      <div className="formRow">
        <label>Package path</label>
        <input value={path} onChange={(e) => setPath(e.target.value)} />
      </div>

      <button onClick={runImport}>Import</button>

      <div className="quickImport">
        <h3>Quick import</h3>

        <button onClick={() => setPath("packages/isuret.core.levels")}>
          isuret.core.levels
        </button>

        <button onClick={() => setPath("packages/isuret.core.domains")}>
          isuret.core.domains
        </button>
      </div>

      {message && <p className="success">{message}</p>}
      {error && <p className="error">{error}</p>}
    </section>
  );
}
