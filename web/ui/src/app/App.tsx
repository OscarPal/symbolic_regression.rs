import React from "react";
import { useEffect } from "react";
import { useSessionStore } from "../state/sessionStore";
import { EnterData } from "./panes/EnterData";
import { CleanSmooth } from "./panes/CleanSmooth";
import { ModelingTask } from "./panes/ModelingTask";
import { SearchSolutions } from "./panes/SearchSolutions";

export function App(): React.ReactElement {
  const tab = useSessionStore((s) => s.tab);
  const setTab = useSessionStore((s) => s.setTab);
  const loadWasmMetadata = useSessionStore((s) => s.loadWasmMetadata);
  const wasmLoaded = useSessionStore((s) => s.wasmLoaded);

  useEffect(() => {
    void loadWasmMetadata();
  }, [loadWasmMetadata]);

  return (
    <div className="app">
      <header className="topbar">
        <div className="title">Symbolic Regression (Web)</div>
        <div className="tabs">
          <button className={tab === "data" ? "tab active" : "tab"} onClick={() => setTab("data")}>
            Enter Data
          </button>
          <button className={tab === "clean" ? "tab active" : "tab"} onClick={() => setTab("clean")}>
            Clean / Smooth
          </button>
          <button className={tab === "task" ? "tab active" : "tab"} onClick={() => setTab("task")}>
            Modeling Task
          </button>
          <button className={tab === "search" ? "tab active" : "tab"} onClick={() => setTab("search")}>
            Search + Solutions
          </button>
        </div>
        <div className="statusChip">{wasmLoaded ? "WASM ready" : "Loading WASM…"}</div>
      </header>

      <main className="main">
        {tab === "data" && <EnterData />}
        {tab === "clean" && <CleanSmooth />}
        {tab === "task" && <ModelingTask />}
        {tab === "search" && <SearchSolutions />}
      </main>
    </div>
  );
}

