import React from "react";

export interface FilterViewProps { }

const FilterView: React.FC<FilterViewProps> = () => {
  return (
    <div
      style={{
        fontWeight: "500",
      }}
    >
      TODO: <br />
      <span
        style={{
          color: "#00000066",
          fontWeight: "normal",
          fontSize: "24px"
        }}
      >
        Filter View
      </span>
    </div>
  );
};

export default FilterView;
