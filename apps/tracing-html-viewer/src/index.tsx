import ReactDOM from "react-dom";
import App from "./viewer/App";

const root = document.getElementById("root")!;
const traceDataEl = document.getElementById("trace-data")!;

console.log(traceDataEl);

ReactDOM.render(<App />, root);
