import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import './App.css'

function App() {
  const [tunnels, setTunnels] = useState<string[]>([]);
  const [configContent, setConfigContent] = useState<string | null>(null);
  const [activeTunnels, setActiveTunnels] = useState<string[]>([]);

  useEffect(() => {
    refreshTunnels();
  }, []);

  async function refreshTunnels() {
    try {
      const fileNames: string[] = await invoke("list_tunnels");
      setTunnels(fileNames);

      const active: string[] = await invoke("get_active_tunnels");
      setActiveTunnels(active);
    } catch (error) {
      console.error("Ошибка при получении списка туннелей:", error);
    }
  }

  function isActive(tunnel: string) {
    return activeTunnels.includes(tunnel.split('.')[0]);
  }

  function viewConfig(fileName: string) {
    try {
      invoke<string>("read_config_file", { fileName }).then((content)=>
        setConfigContent(content)
      );

    } catch (error) {
      console.error("Ошибка при чтении файла:", error);
    }
  }

  function deleteConfig(fileName: string) {
    try {
      invoke("delete_config_file", { fileName }).then(async () => 
        await refreshTunnels()
      )
    } catch (error) {
      console.error("Ошибка при удалении файла:", error);
    }
  }

  function toggleTunnel(fileName: string) {
    try {
      const action = isActive(fileName) ? "down" : "up";
      invoke("toggle_tunnel", { fileName, action }).then(() => 
        refreshTunnels()
      )
    } catch (error) {
      console.error(`Ошибка при переключении состояния туннеля:`, error);
    }
  }
  
  async function handleFileUpload(event: React.ChangeEvent<HTMLInputElement>) {
    const file = event.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = () => {
      if (reader.result) {
        const content = reader.result.toString();
        invoke("save_config_file", { fileName: file.name, content }).then(() =>
          refreshTunnels()
        )
      }
    };
    reader.readAsText(file);
  }

  return (
    <div>
      <div className="tunels">
        <label className="file-upload">
          <input type="file" onChange={handleFileUpload} />
          Choise file
        </label>
        <div className="tunels-list">
          {tunnels.map((fileName) => (
            <div key={fileName} onClick={() => viewConfig(fileName)}>
              <span className={`active-indicator ${isActive(fileName) ? "on" : "off"}`}/>
              {fileName.split('.')[0]}{" "}
              <button
                className={isActive(fileName) ? "stop-btn" : "start-btn"}
                onClick={() => toggleTunnel(fileName)}
              >
                {isActive(fileName) ? "Stop" : "Start"}
              </button>
              <button onClick={() => deleteConfig(fileName)}>X</button>
            </div>
          ))}
        </div>
      </div>

      <div className="config-content">
      {configContent && (
        <>
          <h2>Config Content</h2>
          <pre>{configContent}</pre>
        </>
      )}
      </div>
    </div>
  );
}

export default App;