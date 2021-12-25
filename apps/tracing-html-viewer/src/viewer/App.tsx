import React from "react";
import { SpanDecls, SpanTraceData } from "../types";
import SpanView from "./SpanTraceView";

export interface AppProps {
  spanDecls: SpanDecls;
  root: SpanTraceData;
}

const App: React.FC<AppProps> = ({ spanDecls, root }) => {
  return (
    <div>
      <SpanView spanDecls={spanDecls} data={root} />
    </div>
  );
};

export default App;
