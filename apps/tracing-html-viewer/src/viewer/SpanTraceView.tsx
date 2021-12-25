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
    <div
      style={{
        display: "grid",
        gridTemplateColumns: "75px 1fr",
      }}
    >
      {id && (
        <div
          style={{
            gridColumn: 1,
            gridRow: 1,
            display: "flex",
            flexDirection: "row",
            alignItems: "center",
          }}
        >
          {id && data.spans.length > 0 && (
            <>{!expanded ? <ExpandMoreIcon /> : <ChevronRightIcon />}</>
          )}

          {id && (
            <div style={{ display: "inline" }}>
              <b>{spanDecls[id]?.metadata.name ?? `<Unknown>`}</b>
            </div>
          )}
        </div>
      )}

      <div
        style={{
          gridColumn: !!id ? 2 : 1,
          gridRow: !!id ? 2 : 1,
        }}
      >
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

        <div>
          <p>
            Started: <span>{data.enteredAt ?? data.createdAt}</span>
          </p>
          <p>Ended: {data.exitedAt ?? `<Unknown>`}</p>
        </div>

        {data.spans.length > 0 && (
          <div>
            {data.spans.map(([id, data]) => (
              <SpanTraceView
                spanDecls={spanDecls}
                id={id}
                data={data}
              ></SpanTraceView>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

export default SpanTraceView;
