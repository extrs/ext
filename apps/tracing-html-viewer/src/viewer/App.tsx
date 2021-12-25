import React from "react";
import { SpanDecls, SpanTraceData } from "../types";
import SpanTraceView from "./SpanTraceView";

export interface AppProps {
  spanDecls: SpanDecls;
  root: SpanTraceData;
}

const App: React.FC<AppProps> = ({ spanDecls, root }) => {
  return (
    <div>
      <SpanTraceView spanDecls={spanDecls} data={root} />
    </div>
  );
};

export default App;
