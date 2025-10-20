import React, { useState, useEffect } from "react";
import Layout from "@theme/Layout";

function PlaygroundWrapper(): React.ReactElement {
  const [PlaygroundClient, setPlaygroundClient] =
    useState<React.ComponentType | null>(null);

  useEffect(() => {
    // Only import the client component on the browser
    import("../components/PlaygroundClient").then((mod) => {
      setPlaygroundClient(() => mod.default);
    });
  }, []);

  if (!PlaygroundClient) {
    return (
      <div style={{ padding: "2rem", textAlign: "center" }}>
        Loading playground...
      </div>
    );
  }

  return <PlaygroundClient />;
}

export default function Playground(): React.ReactElement {
  return (
    <Layout
      title="Playground"
      description="Interactive Sudoku Witness Encryption Demo"
    >
      <PlaygroundWrapper />
    </Layout>
  );
}
