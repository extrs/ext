import React from "react";
import ReactDOM from "react-dom";
import App from "./viewer/App";

const root = document.getElementById("root")!;
const traceDataEl = document.getElementById("trace-data")!;

const traceDataStr = traceDataEl.innerText;

const { spanDecls, root: events } = JSON.parse(traceDataStr);
console.log(spanDecls);
console.log(events);

ReactDOM.render(<App events={events} />, root);
