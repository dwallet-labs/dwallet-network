// @ts-check
// `@type` JSDoc annotations allow editor autocompletion and type checking
// (when paired with `@ts-check`).
// There are various equivalent ways to declare your Docusaurus config.
// See: https://docusaurus.io/docs/api/docusaurus-config

import {themes as prismThemes} from 'prism-react-renderer';

// const lightCodeTheme = require("prism-react-renderer/themes/github");
// const darkCodeTheme = require("prism-react-renderer/themes/nightOwl");
// const math = require("remark-math");
// const katex = require("rehype-katex");
// require("dotenv").config();

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'dWallet Docs',
  tagline: 'The dWallet Network Documentation',
  favicon: 'img/dwallet 443 sizes-14.png',

  // Set the production url of your site here
  url: 'https://your-docusaurus-site.example.com',
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: '/',

  onBrokenLinks: 'ignore',
  onBrokenMarkdownLinks: 'warn',

  // Even if you don't use internationalization, you can use this field to set
  // useful metadata like html lang. For example, if your site is Chinese, you
  // may want to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },
  markdown: {
    mermaid: true,
  },
  presets: [
    [
      '@docusaurus/preset-classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      {
        docs: {
          routeBasePath: '/',
          sidebarPath: './sidebars.js',
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://github.com/dwallet-labs/dwallet-network/tree/main/docs',
          // remarkPlugins: [
          //   math,
          //   require("@docusaurus/remark-plugin-npm2yarn"),
          //   { sync: true, converters: ["npm", "yarn", "pnpm"] },
          // ],
          // rehypePlugins: [katex],
          },
        blog: false,
        theme: {
          customCss: './src/css/custom.css',
        },
      },
    ],
  ],
  // stylesheets: [
  //   {
  //     href: "https://cdn.jsdelivr.net/npm/katex@0.13.24/dist/katex.min.css",
  //     type: "text/css",
  //     integrity:
  //       "sha384-odtC+0UGzzFL/6PNoE8rX/SPcQDXBJ+uRepguP4QkPCm2LBxH3FA3y+fKSiJ+AmM",
  //     crossorigin: "anonymous",
  //   },
  // ],
  //themes: ["@docusaurus/theme-mermaid"],
  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      // Replace with your project's social card
      image: 'img/dwallet-social-card.png',
      navbar: {
        title: 'dWallet Docs',
        logo: {
          alt: 'dWallet Logo',
          src: 'img/dwallet 443 sizes-14.png',
        },
        items: [
          {
            href: 'https://github.com/dwallet-labs/dwallet-network',
            label: 'GitHub',
            position: 'right',
          },
        ],
      },
      footer: {
        style: 'dark',
        links: [
          {
            title: 'Community',
            items: [
              {
                label: 'Discord',
                href: 'https://discord.gg/dWallet',
              },
              {
                label: 'Twitter',
                href: 'https://twitter.com/dWalletNetwork',
              },
            ],
          },
          {
            title: 'More',
            items: [
              {
                label: 'Blog',
                href: 'https://dwallet.io/blog',
              },
              {
                label: 'GitHub',
                href: 'https://github.com/dwallet-labs/dwallet-network',
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} dWallet Labs, ltd. Built with Docusaurus.`,
      },
      prism: {
        theme: prismThemes.github,
        darkTheme: prismThemes.dracula,
        //additionalLanguages: ["rust", "typescript", "toml"],
      },
    }),
};

export default config;
