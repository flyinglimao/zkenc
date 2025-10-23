import type { ReactNode } from "react";
import clsx from "clsx";
import Heading from "@theme/Heading";
import styles from "./styles.module.css";

type FeatureItem = {
  title: string;
  Svg: React.ComponentType<React.ComponentProps<"svg">>;
  description: ReactNode;
};

const FeatureList: FeatureItem[] = [
  {
    title: "Encrypt to Statements, Not Keys",
    Svg: require("@site/static/img/undraw_security.svg").default,
    description: (
      <>
        Witness encryption allows you to encrypt data to a computational
        statement. Anyone with a valid solution can decrypt—no key exchange, no
        trusted third parties. Just verifiable computation.
      </>
    ),
  },
  {
    title: "Built on Circom",
    Svg: require("@site/static/img/undraw_programming.svg").default,
    description: (
      <>
        Define your decryption conditions using Circom circuits. From simple
        puzzles to complex access policies—if you can write it as a constraint,
        you can encrypt to it. Full R1CS support included.
      </>
    ),
  },
  {
    title: "Works Everywhere",
    Svg: require("@site/static/img/undraw_web_devices.svg").default,
    description: (
      <>
        Use it in the terminal with the CLI, in Node.js applications, or
        directly in the browser. Powered by Rust and WASM for high performance
        across all platforms.
      </>
    ),
  },
];

function Feature({ title, Svg, description }: FeatureItem) {
  return (
    <div className={clsx("col col--4")}>
      <div className="text--center">
        <Svg className={styles.featureSvg} role="img" />
      </div>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{title}</Heading>
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
