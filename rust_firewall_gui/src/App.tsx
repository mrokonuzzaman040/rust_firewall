import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

type LogTuple = [number, string, string, string, number];

function App() {
  const [logs, setLogs] = useState<LogTuple[]>([]);

  // Fetch logs on mount
  useEffect(() => {
    refreshLogs();
  }, []);

  async function refreshLogs() {
    try {
      const result = await invoke<LogTuple[]>("fetch_logs");
      setLogs(result);
    } catch (err) {
      console.error("Error fetching logs:", err);
    }
  }

  async function handleStartCapture() {
    try {
      await invoke("start_capture_cmd");
      alert("Capture started!");
    } catch (err) {
      console.error(err);
    }
  }

  return (
    <div style={{ padding: "20px" }}>
      <h1>Rust Firewall GUI</h1>
      <button onClick={refreshLogs}>Refresh Logs</button>
      <button onClick={handleStartCapture} style={{ marginLeft: "10px" }}>
        Start Capture
      </button>

      <table style={{ marginTop: "20px", border: "1px solid #ccc" }}>
        <thead>
          <tr>
            <th>ID</th>
            <th>Timestamp</th>
            <th>Connection</th>
            <th>Protocol</th>
            <th>Bytes</th>
          </tr>
        </thead>
        <tbody>
          {logs.map(([id, timestamp, connection, protocol, bytes]) => (
            <tr key={id}>
              <td>{id}</td>
              <td>{timestamp}</td>
              <td>{connection}</td>
              <td>{protocol}</td>
              <td>{bytes}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default App;
