import React from "react";
import ReactDOM from "react-dom";
import App from "./viewer/App";

const root = document.getElementById("root")!;
const traceDataEl = document.getElementById("trace-data")!;

const traceDataStr = traceDataEl.innerText;

const events = traceDataStr
  .split("\n")
  .filter((v) => !!v)
  .map((v) => JSON.parse(v));
console.log(events);

ReactDOM.render(<App events={events} />, root);
