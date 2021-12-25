import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import ChevronRightIcon from "@mui/icons-material/ChevronRight";
import React, { useState } from "react";
import { SpanDecls, SpanTraceData } from "../types";
import Grid from "@mui/material/Grid";

export interface SpanViewProps {
  spanDecls: SpanDecls;
  id?: number;
  data: SpanTraceData;
}

const SpanTraceView: React.FC<SpanViewProps> = ({ spanDecls, id, data }) => {
  const [expanded, setExpanded] = useState(true);

  return (
    <div>
      {id && <div>{spanDecls[id]?.metadata.name ?? `<Unknown>`}</div>}

      {data.enteredAt}
      {data.spans.length > 0 && (
        <>
          <Grid container direction="row">
            <Grid item>
              {!expanded ? <ExpandMoreIcon /> : <ChevronRightIcon />}
            </Grid>
            <Grid item>
              {data.spans.map(([id, data]) => (
                <SpanTraceView
                  spanDecls={spanDecls}
                  id={id}
                  data={data}
                ></SpanTraceView>
              ))}
            </Grid>
          </Grid>
        </>
      )}
    </div>
  );
};

export default SpanTraceView;
