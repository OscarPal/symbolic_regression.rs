import React from "react";

export function CleanSmooth(): React.ReactElement {
  return (
    <div className="pane">
      <div className="card">
        <div className="cardTitle">Clean / Smooth</div>
        <div className="muted">
          Placeholder: this pane is scaffolded for a future preprocessing pipeline (drop NaNs, standardize, smoothing, outlier
          handling).
        </div>
      </div>
    </div>
  );
}

