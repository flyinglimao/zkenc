/// <reference types="@docusaurus/theme-classic" />

declare module "*.module.css" {
  const classes: { readonly [key: string]: string };
  export default classes;
}

declare module "*.css" {
  const content: { readonly [key: string]: string };
  export default content;
}
