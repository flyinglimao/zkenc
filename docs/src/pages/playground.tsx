import React, { useState, useEffect } from "react";
import Layout from "@theme/Layout";
import Translate, { translate } from "@docusaurus/Translate";

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
        <Translate id="playground.loading">Loading playground...</Translate>
      </div>
    );
  }

  return <PlaygroundClient />;
}

export default function Playground(): React.ReactElement {
  return (
    <Layout
      title={translate({
        id: "playground.title",
        message: "Playground",
      })}
      description={translate({
        id: "playground.description",
        message: "Interactive Sudoku Witness Encryption Demo",
      })}
    >
      <PlaygroundWrapper />
    </Layout>
  );
}
