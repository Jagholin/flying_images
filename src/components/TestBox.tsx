import React from "react";

function TestBox({
  min_width,
  min_height,
  grow,
  children,
}: {
  min_width?: number;
  min_height?: number;
  grow?: boolean;
  children?: any;
}) {
  return (
    <div
      style={{
        minWidth: min_width,
        minHeight: min_height,
        outline: "1px solid black",
        flexGrow: grow ? 1 : undefined,
      }}
    >
      Test
      {children}
    </div>
  );
}

export default TestBox;
