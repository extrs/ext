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

const EventView: React.FC<EventViewProps> = ({ event }) => {
  return (
    <div>
      <Accordion>
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
