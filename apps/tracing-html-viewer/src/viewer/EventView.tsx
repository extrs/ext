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
  const levelTextStyle = {
    color: "#000000",
    fontWeight: "500",
    fontSize: "24px",
    marginBottom: "10px"
  }
  const bodyTextStyle = {
    color: "#000000",
    fontWeight: "500",
    fontSize: "18px",
    marginBottom: "10px"
  }
  return (
    <div style={{ display: "inline-block" }}>
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
            <div style={levelTextStyle}>Level {event.metadata.level}</div>
            <div>Name:{" "}
              <span style={bodyTextStyle}>
                {event.metadata.name}
              </span>
            </div>
            <div>Target:{" "}
              <span style={bodyTextStyle}>
                {event.metadata.target}
              </span>
            </div>
            <div>File:{" "}
              <span style={bodyTextStyle}>
                {event.metadata.file}
              </span>
            </div>
            <div>Line:{" "}
              <span style={bodyTextStyle}>
                {event.metadata.line}
              </span>
            </div>
            <div>Module path:{" "}
              <span style={bodyTextStyle}>
                {event.metadata.modulePath}
              </span>
            </div>
          </Typography>
        </AccordionDetails>
      </Accordion>
    </div>
  );
};

export default EventView;
