import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

function App() {
  const [file, setFile] = useState<File | null>(null);
  const [configContent, setConfigContent] = useState("");
  const [tunnels, setTunnels] = useState<string[]>([]);

  useEffect(() => {
    invoke<string[]>("list_tunnels").then(setTunnels);
  }, []);

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const selectedFile = event.target.files?.[0];
    if (selectedFile) {
      setFile(selectedFile);
      const reader = new FileReader();
      reader.onload = () => setConfigContent(reader.result as string);
      reader.readAsText(selectedFile);
    }
  };

  const saveConfig = async () => {
    if (file && configContent) {
      await invoke("save_config_file", { 
        fileName: file.name, 
        content: configContent 
      });
      setTunnels([...tunnels, file.name]);
    }
  };

  const toggleTunnel = async (fileName: string, action: string) => {
    await invoke("toggle_tunnel", { fileName, action });
  };

  return (
    <div>
      <input type="file" onChange={handleFileChange} />
      <button onClick={saveConfig} disabled={!file}>
        Save Config
      </button>
      <h2>Active Tunnels</h2>
      <ul>
        {tunnels.map((tunnel) => (
          <li key={tunnel}>
            {tunnel}
            <button onClick={() => toggleTunnel(tunnel, "up")}>Start</button>
            <button onClick={() => toggleTunnel(tunnel, "down")}>Stop</button>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default App;