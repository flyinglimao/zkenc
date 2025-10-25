import type { ReactNode } from "react";
import clsx from "clsx";
import Heading from "@theme/Heading";
import Translate, { translate } from "@docusaurus/Translate";
import styles from "./styles.module.css";

type FeatureItem = {
  key: string;
  title: string;
  Svg: React.ComponentType<React.ComponentProps<"svg">>;
  description: ReactNode;
};

const FeatureList: FeatureItem[] = [
  {
    key: "encrypt",
    title: "Encrypt to Statements, Not Keys",
    Svg: require("@site/static/img/undraw_security.svg").default,
    description: (
      <Translate id="homepage.features.encrypt.description">
        Witness encryption allows you to encrypt data to a computational
        statement. Anyone with a valid solution can decrypt—no key exchange, no
        trusted third parties. Just verifiable computation.
      </Translate>
    ),
  },
  {
    key: "circom",
    title: "Built on Circom",
    Svg: require("@site/static/img/undraw_programming.svg").default,
    description: (
      <Translate id="homepage.features.circom.description">
        Define your decryption conditions using Circom circuits. From simple
        puzzles to complex access policies—if you can write it as a constraint,
        you can encrypt to it. Full R1CS support included.
      </Translate>
    ),
  },
  {
    key: "everywhere",
    title: "Works Everywhere",
    Svg: require("@site/static/img/undraw_web_devices.svg").default,
    description: (
      <Translate id="homepage.features.everywhere.description">
        Use it in the terminal with the CLI, in Node.js applications, or
        directly in the browser. Powered by Rust and WASM for high performance
        across all platforms.
      </Translate>
    ),
  },
];

function Feature({ key, title, Svg, description }: FeatureItem) {
  // Render title based on key
  const titleElement = (() => {
    switch (key) {
      case "encrypt":
        return (
          <Translate id="homepage.features.encrypt.title">
            Encrypt to Statements, Not Keys
          </Translate>
        );
      case "circom":
        return (
          <Translate id="homepage.features.circom.title">
            Built on Circom
          </Translate>
        );
      case "everywhere":
        return (
          <Translate id="homepage.features.everywhere.title">
            Works Everywhere
          </Translate>
        );
      default:
        return title;
    }
  })();

  return (
    <div className={clsx("col col--4")}>
      <div className="text--center">
        <Svg className={styles.featureSvg} role="img" />
      </div>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{titleElement}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): ReactNode {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
