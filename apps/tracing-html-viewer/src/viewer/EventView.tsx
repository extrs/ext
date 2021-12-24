import React from "react";
import { TraceEvent } from "../types";

export interface EventViewProps {
  event: TraceEvent;
}

const EventView: React.FC<EventViewProps> = ({ event }) => {
  return (
    <div>
      {event.fields.message}

      {}
    </div>
  );
};

export default EventView;
