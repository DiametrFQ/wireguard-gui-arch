import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";


const App = () => {
  const [tunnelName, setTunnelName] = useState("");
  const [fileContent, setFileContent] = useState("");

  const handleFileSelection = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (file) {
      const content = await file.text();
      setFileContent(content); // Сохраняем содержимое файла
    }
  };

  const handleAddTunnel = async () => {
    console.log({tunnelName, fileContent})
    if (!tunnelName || !fileContent) {
      alert("Please provide a name and select a config file.");
      return;
    }

    try {
      await invoke('add_tunnel', { configContent: fileContent, tunnelName });
      alert('Tunnel added successfully!');
    } catch (error) {
      console.error('Failed to add tunnel:', error);
    }
  };

  return (
    <div>
      <input
        type="text"
        value={tunnelName}
        onChange={(e) => setTunnelName(e.target.value)}
        placeholder="Enter Tunnel Name"
      />
      <input
        type="file"
        onChange={handleFileSelection}
      />
      <button onClick={handleAddTunnel}>Add Tunnel</button>
    </div>
  );
};

export default App;
