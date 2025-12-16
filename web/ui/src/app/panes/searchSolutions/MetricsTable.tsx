import React from "react";
import type { WasmEvalResult } from "../../../types/srTypes";
import { formatSci } from "./plotUtils";

export function MetricsTable({ m }: { m: WasmEvalResult["metrics"] }): React.ReactElement {
  return (
    <table className="table tight">
      <tbody>
        <tr>
          <td>n</td>
          <td className="mono">{m.n}</td>
        </tr>
        <tr>
          <td>mse</td>
          <td className="mono">{formatSci(m.mse)}</td>
        </tr>
        <tr>
          <td>mae</td>
          <td className="mono">{formatSci(m.mae)}</td>
        </tr>
        <tr>
          <td>rmse</td>
          <td className="mono">{formatSci(m.rmse)}</td>
        </tr>
        <tr>
          <td>r2</td>
          <td className="mono">{formatSci(m.r2)}</td>
        </tr>
        <tr>
          <td>corr</td>
          <td className="mono">{formatSci(m.corr)}</td>
        </tr>
        <tr>
          <td>min |err|</td>
          <td className="mono">{formatSci(m.min_abs_err)}</td>
        </tr>
        <tr>
          <td>max |err|</td>
          <td className="mono">{formatSci(m.max_abs_err)}</td>
        </tr>
      </tbody>
    </table>
  );
}

