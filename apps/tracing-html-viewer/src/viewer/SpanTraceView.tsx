import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import ChevronRightIcon from "@mui/icons-material/ChevronRight";
import React, { useState } from "react";
import { SpanDecls, SpanTraceData } from "../types";

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

      {id && spanDecls[id] && Object.keys(spanDecls[id].attrs).length > 0 && (
        <div>
          <span>Attributes:</span>

          <div>
            {Object.entries(spanDecls[id].attrs).map(([key, value]) => (
              <div key={key}>
                <span>{key}:</span>
                <span>{value}</span>
              </div>
            ))}
          </div>
        </div>
      )}

      {data.enteredAt}
      {data.spans.length > 0 && (
        <div style={{ display: "flex", flexDirection: "row" }}>
          <div>{!expanded ? <ExpandMoreIcon /> : <ChevronRightIcon />}</div>
          <div style={{ flexGrow: 1 }}>
            {data.spans.map(([id, data]) => (
              <SpanTraceView
                spanDecls={spanDecls}
                id={id}
                data={data}
              ></SpanTraceView>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

export default SpanTraceView;
