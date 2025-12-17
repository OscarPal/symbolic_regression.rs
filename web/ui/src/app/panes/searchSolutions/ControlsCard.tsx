import React from "react";
import type { SearchSnapshot } from "../../../types/srTypes";
import type { FitPlotMode } from "./types";

export function ControlsCard(props: {
  canInit: boolean;
  status: string;
  error: string | null;
  snap: SearchSnapshot | null;
  cyclesPerSecond: number | null;

  fitMode: FitPlotMode;
  setFitMode: (m: FitPlotMode) => void;
  canCurve1d: boolean;

  initSearch: () => void;
  start: () => void;
  pause: () => void;
  reset: () => void;
}): React.ReactElement {
  return (
    <div className="card">
      <div className="cardTitle">Controls</div>
      <div className="row">
        <button onClick={props.initSearch} disabled={!props.canInit} data-testid="search-init">
          Initialize
        </button>
        <button onClick={props.start} disabled={props.status !== "ready" && props.status !== "paused"} data-testid="search-start">
          Start / Resume
        </button>
        <button onClick={props.pause} disabled={props.status !== "running"}>
          Pause
        </button>
        <button onClick={props.reset}>Reset</button>

        <label className="field">
          <div className="label">fit plot</div>
          <select value={props.fitMode} onChange={(e) => props.setFitMode(e.target.value as FitPlotMode)}>
            <option value="auto">Auto</option>
            <option value="parity">Parity (y vs ŷ)</option>
            <option value="curve_1d" disabled={!props.canCurve1d}>
              1D curve (x vs y)
            </option>
          </select>
        </label>

        <div className="statusLine">
          <span className="statusChip" data-testid="search-status">
            {props.status}
          </span>
          {props.error && <span className="errorText">{props.error}</span>}
          {props.snap && (
            <span className="muted">
              cycles {props.snap.cycles_completed}/{props.snap.total_cycles} (
              {props.snap.total_cycles > 0 ? ((100 * props.snap.cycles_completed) / props.snap.total_cycles).toFixed(1) : "0"}%), evals=
              {props.snap.total_evals}
              {props.cyclesPerSecond != null ? `, ${props.cyclesPerSecond.toFixed(1)} cyc/s` : ""}
            </span>
          )}
        </div>
      </div>
    </div>
  );
}
