import React from "react";
import { TraceEvent } from "../types";

export interface AppProps {
  events: TraceEvent[];
}

const App: React.FC<AppProps> = ({ events }) => {
  return (
    <div>
      {events.map((event, i) => (
        <div key={i}>{event.fields.message}</div>
      ))}
    </div>
  );
};

export default App;
