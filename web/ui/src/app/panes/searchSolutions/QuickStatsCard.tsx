import React from "react";
import type { EquationSummary, WasmEvalResult } from "../../../types/srTypes";
import { copyToClipboard } from "./plotUtils";
import { MetricsTable } from "./MetricsTable";

export function QuickStatsCard(props: {
  selectedSummary: EquationSummary | null;
  evalTrain?: WasmEvalResult;
  evalVal?: WasmEvalResult;
  hasVal: boolean;
}): React.ReactElement {
  return (
    <div className="card gridCell">
      <div className="cardTitle">Quick stats</div>
      {!props.selectedSummary ? (
        <div className="muted">Select a solution to compute metrics.</div>
      ) : (
        <>
          <div className="mono bigEq" data-testid="selected-equation">
            {props.selectedSummary.equation}
          </div>
          <div className="row">
            <button onClick={() => copyToClipboard(props.selectedSummary!.equation)}>Copy equation</button>
          </div>
          <div className="subTitle">Train</div>
          {props.evalTrain ? (
            <MetricsTable m={props.evalTrain.metrics} />
          ) : (
            <div className="muted" data-testid="no-metrics">
              No metrics yet (click solution to evaluate).
            </div>
          )}
          {props.hasVal && (
            <>
              <div className="subTitle">Validation</div>
              {props.evalVal ? <MetricsTable m={props.evalVal.metrics} /> : <div className="muted">No metrics yet.</div>}
            </>
          )}
        </>
      )}
    </div>
  );
}

