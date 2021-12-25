import React from "react";
import { SpanDecl, SpanTraceData, TraceEvent } from "../types";

export interface AppProps {
  spanDecls: { [id: number]: SpanDecl };
  root: SpanTraceData;
}

const App: React.FC<AppProps> = ({ spanDecls, root }) => {
  return <div></div>;
};

export default App;
