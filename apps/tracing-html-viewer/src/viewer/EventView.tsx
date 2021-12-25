import React from "react";
import { TraceEvent } from "../types";

export interface EventViewProps {
  event: TraceEvent;
}

const EventView: React.FC<EventViewProps> = ({ event }) => {
  return (
    <div>
      {event.fields.message}
      <div>Level {event.metadata.level}</div>
      <div>Name: {event.metadata.name}</div>
      <div>Target: {event.metadata.target}</div>
      <div>File: {event.metadata.file}</div>
      <div>Line: {event.metadata.line}</div>
      <div>Module path: {event.metadata.modulePath}</div>
    </div>
  );
};

export default EventView;
