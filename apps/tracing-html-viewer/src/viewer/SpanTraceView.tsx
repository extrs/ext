import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import ChevronRightIcon from "@mui/icons-material/ChevronRight";
import TreeView from "@mui/lab/TreeView";
import React from "react";
import { SpanDecls, SpanTraceData } from "../types";
import TreeItem from "@mui/lab/TreeItem";

export interface SpanViewProps {
  spanDecls: SpanDecls;
  id?: number;
  data: SpanTraceData;
}

const SpanView: React.FC<SpanViewProps> = ({ spanDecls, id, data }) => {
  const renderTree = (id: number, data: SpanTraceData) => (
    <TreeItem
      key={id}
      nodeId={id.toString()}
      label={spanDecls[id]?.metadata.name ?? "<Unknown>"}
    >
      {data.spans.map(([id, data]) => renderTree(id, data))}
    </TreeItem>
  );

  return (
    <div>
      {id && spanDecls[id] && <div>{spanDecls[id].metadata.name}</div>}

      {data.enteredAt}
      <div>
        <TreeView
          defaultCollapseIcon={<ExpandMoreIcon />}
          defaultExpanded={["root"]}
          defaultExpandIcon={<ChevronRightIcon />}
          sx={{
            height: 110,
            flexGrow: 1,
            maxWidth: 400,
            overflowY: "auto",
          }}
        >
          {renderTree(0, data)}
        </TreeView>
      </div>
    </div>
  );
};

export default SpanView;
