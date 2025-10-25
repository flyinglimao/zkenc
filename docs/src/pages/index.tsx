import type { ReactNode } from "react";
import clsx from "clsx";
import Link from "@docusaurus/Link";
import useDocusaurusContext from "@docusaurus/useDocusaurusContext";
import Layout from "@theme/Layout";
import HomepageFeatures from "@site/src/components/HomepageFeatures";
import Heading from "@theme/Heading";
import Translate, { translate } from "@docusaurus/Translate";

import styles from "./index.module.css";

function HomepageHeader() {
  const { siteConfig } = useDocusaurusContext();
  return (
    <header className={clsx("hero hero--primary", styles.heroBanner)}>
      <div className="container">
        <Heading as="h1" className="hero__title">
          {siteConfig.title}
        </Heading>
        <p className="hero__subtitle">{siteConfig.tagline}</p>
        <div className={styles.buttons}>
          <Link
            className="button button--secondary button--lg"
            to="/playground"
          >
            <Translate id="homepage.buttons.playground">
              🎮 Try Playground
            </Translate>
          </Link>
          <Link className="button button--outline button--lg" to="/docs/intro">
            <Translate id="homepage.buttons.docs">
              📖 Read Documentation
            </Translate>
          </Link>
        </div>
      </div>
    </header>
  );
}

export default function Home(): ReactNode {
  return (
    <Layout
      title={translate({
        id: "homepage.title",
        message: "Witness Encryption for Circom Circuits",
      })}
      description={translate({
        id: "homepage.description",
        message:
          "Conditional encryption for Circom circuits. Encrypt to computational statements, decrypt with valid witnesses. Available as CLI tool and JavaScript library.",
      })}
    >
      <HomepageHeader />
      <main>
        <HomepageFeatures />
      </main>
    </Layout>
  );
}
