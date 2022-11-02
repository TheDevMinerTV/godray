import { register } from "@tauri-apps/api/globalShortcut";
import { appWindow } from "@tauri-apps/api/window";
import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./style.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);

document.onkeyup = (e) => {
  if (e.key === "Escape") {
    appWindow.hide();
  }
};

await register("Alt+Space", async () => {
  await appWindow.show();

  // TODO: Focus the input
});
