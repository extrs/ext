import React from "react";
import { SpanDecls, SpanTraceData } from "../types";
import FilterView from "./FilterView";
import SpanTraceView from "./SpanTraceView";

export interface AppProps {
  spanDecls: SpanDecls;
  root: SpanTraceData;
}

const App: React.FC<AppProps> = ({ spanDecls, root }) => {
  return (
    <div
      style={{
        overflowX: "auto",
        width: "100%",
        display: "flex",
        flexDirection: "column",
        margin: "20px",
      }}
    >
      <div>
        <FilterView />
      </div>
      <SpanTraceView spanDecls={spanDecls} data={root} />
    </div>
  );
};

export default App;
