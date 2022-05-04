import React from "react";
import ReactDOM from "react-dom";
import App from "./viewer/App";

const rootEl = document.getElementById("root")!;
const traceDataEl = document.getElementById("trace-data")!;

const traceDataStr = traceDataEl.innerText;

const { spanDecls, root } = JSON.parse(traceDataStr);

ReactDOM.render(<App root={root} spanDecls={spanDecls} />, rootEl);
