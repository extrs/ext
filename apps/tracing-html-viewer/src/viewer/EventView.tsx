import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import Accordion from "@mui/material/Accordion";
import AccordionDetails from "@mui/material/AccordionDetails";
import AccordionSummary from "@mui/material/AccordionSummary";
import Typography from "@mui/material/Typography";
import React from "react";
import { TraceEvent } from "../types";

export interface EventViewProps {
  event: TraceEvent;
}

function colorFor(level: string) {
  switch (level) {
    case "INFO":
      return "#00bcd4";
    case "ERROR":
      return "#f44336";
    case "DEBUG":
      return "#4caf50";
    case "WARN":
      return "#ffc107";
    case "TRACE":
      return "#9e9e9e";
  }
  throw new Error(`Unknown level: ${level}`);
}

const EventView: React.FC<EventViewProps> = ({ event }) => {
  return (
    <div>
      <Accordion style={{ background: colorFor(event.metadata.level) }}>
        <AccordionSummary
          expandIcon={<ExpandMoreIcon />}
          aria-controls="panel1a-content"
          id="panel1a-header"
        >
          <Typography>{event.fields.message}</Typography>
        </AccordionSummary>
        <AccordionDetails>
          <Typography>
            <div>Level {event.metadata.level}</div>
            <div>Name: {event.metadata.name}</div>
            <div>Target: {event.metadata.target}</div>
            <div>File: {event.metadata.file}</div>
            <div>Line: {event.metadata.line}</div>
            <div>Module path: {event.metadata.modulePath}</div>
          </Typography>
        </AccordionDetails>
      </Accordion>
    </div>
  );
};

export default EventView;
