import { defineConfig } from "vitepress";
import { withMermaid } from 'vitepress-plugin-mermaid';
import path from "path";

const projectRootDir = path.resolve(__dirname);

export default withMermaid(defineConfig({
  title: "CCDI Data Federation API",
  description:
    "The documentation site for the Childhood Cancer Data Initiative Federation API.",
  base: "/ccdi-federation-api/",
  themeConfig: {
    nav: [
      { text: "Home", link: "/" },
      { text: "Blog", link: "/blog" },
    ],

    sidebar: [
      {
        text: "Overview",
        link: "/overview",
        items: [
          {
            text: "Definitions",
            link: "/concepts/definitions",
          },
          {
            text: "Security Requirements",
            link: "/concepts/security",
          },
          {
            text: "Incomplete Data and Nullity",
            link: "/concepts/incomplete-data-and-nullity",
          },
          {
            text: "Invalid Routes",
            link: "/concepts/invalid-routes",
          },
        ],
      },
      {
        text: "Data Model",
        items: [
          {
            text: "Primary Entities",
            link: "/entities/primary",
            items: [
              {
                text: "Subject",
                link: "/entities/primary/subject",
              },
              {
                text: "Sample",
                link: "/entities/primary/sample",
              },
              {
                text: "File",
                link: "/entities/primary/file",
              },
            ],
          },
          {
            text: "Supporting Entities",
            link: "/entities/supporting",
            items: [
              {
                text: "Organization",
                link: "/entities/supporting/organization",
              },
              {
                text: "Namespace",
                link: "/entities/supporting/namespace",
              },
              {
                text: "Guidance on Assigning",
                link: "/entities/supporting/assigning-namespaces-and-organizations",
              },
            ],
            collapsed: true,
          },
          {
            text: "Metadata",
            link: "/metadata",
          },
          {
            text: "Gateways and Links",
            link: "/gateways-and-links",
          },
        ],
      },
      {
        text: "Reference",
        items: [
          {
            text: "API Documentation",
            link: "https://cbiit.github.io/ccdi-federation-api/specification.html",
          },
        ],
      },
      {
        text: "Blog Posts",
        link: "/blog",
        items: [
          {
            text: "Introducing the Federation API",
            link: "/blog/09-25-2024-introducing-the-federation-api",
          },
          {
            text: "Federation Resource API with CPI",
            link: "/blog/07-17-2025-the-federation-api-cpi",
          },
        ],
      },
    ],

    socialLinks: [
      { icon: "github", link: "https://github.com/cbiit/ccdi-federation-api" },
    ],
  },
  vite: {
    optimizeDeps: {
      include: ['@braintree/sanitize-url']
    },
    ssr: {
      noExternal: ["monaco-editor"],
    },
    build: {
      rollupOptions: {
        output: {
          manualChunks(id) {
            if (id.includes("node_modules")) {
              return id
                .toString()
                .split("node_modules/")[1]
                .split("/")[0]
                .toString();
            }
          },
        },
      },
    },
    resolve: {
      alias: {
        "@": projectRootDir,
        dayjs: 'dayjs/'
      },
    },
  },
}));
