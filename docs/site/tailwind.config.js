// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

const defaultTheme = require("tailwindcss/defaultTheme");

module.exports = {
  corePlugins: {
    preflight: false, // disable Tailwind's reset
  },
  content: ["./src/**/*.{js,jsx,ts,tsx}", "./docs/**/*.mdx"], // my markdown stuff is in ../docs, not /src
  darkMode: ["class", '[data-theme="dark"]'], // hooks into docusaurus' dark mode settings
  theme: {
    extend: {
      fontFamily: {
        sans: ["Inter", ...defaultTheme.fontFamily.sans],
        twkeverett: ["Twkeverett"],
      },
      colors: {
        "ika-black": "var(--ika-black)",
        "ika-blue-primary": "rgb(var(--ika-blue-primary)/<alpha-value>)",
        "ika-blue": "var(--ika-blue)",
        "ika-blue-bright": "var(--ika-blue-bright)",
        "ika-blue-light": "var(--ika-blue-light)",
        "ika-blue-lighter": "var(--ika-blue-lighter)",
        "ika-blue-dark": "rgb(var(--ika-blue-dark)/<alpha-value>)",
        "ika-blue-darker": "var(--ika-blue-darker)",
        "ika-hero": "var(--ika-hero)",
        "ika-hero-dark": "var(--ika-hero-dark)",
        "ika-steel": "var(--ika-steel)",
        "ika-steel-dark": "var(--ika-steel-dark)",
        "ika-steel-darker": "var(--ika-steel-darker)",
        "ika-header-nav": "var(--ika-header-nav)",
        "ika-success": "var(--ika-success)",
        "ika-success-dark": "var(--ika-success-dark)",
        "ika-success-light": "var(--ika-success-light)",
        "ika-issue": "var(--ika-issue)",
        "ika-issue-dark": "var(--ika-issue-dark)",
        "ika-issue-light": "var(--ika-issue-light)",
        "ika-warning": "var(--ika-warning)",
        "ika-warning-dark": "var(--ika-warning-dark)",
        "ika-warning-light": "var(--ika-warning-light)",
        "ika-code": "var(--ika-code)",
        "ika-gray": {
          35: "var(--ika-gray-35)",
          40: "var(--ika-gray-40)",
          45: "var(--ika-gray-45)",
          50: "var(--ika-gray-50)",
          55: "var(--ika-gray-55)",
          60: "var(--ika-gray-60)",
          65: "var(--ika-gray-65)",
          70: "var(--ika-gray-70)",
          75: "var(--ika-gray-75)",
          80: "var(--ika-gray-80)",
          85: "var(--ika-gray-85)",
          90: "var(--ika-gray-90)",
          95: "var(--ika-gray-95)",
          100: "var(--ika-gray-100)",
        },
        "ika-grey": {
          35: "var(--ika-gray-35)",
          40: "var(--ika-gray-40)",
          45: "var(--ika-gray-45)",
          50: "var(--ika-gray-50)",
          55: "var(--ika-gray-55)",
          60: "var(--ika-gray-60)",
          65: "var(--ika-gray-65)",
          70: "var(--ika-gray-70)",
          75: "var(--ika-gray-75)",
          80: "var(--ika-gray-80)",
          85: "var(--ika-gray-85)",
          90: "var(--ika-gray-90)",
          95: "var(--ika-gray-95)",
          100: "var(--ika-gray-100)",
        },
        "ika-link-color-dark": "var(--ika-link-color-dark)",
        "ika-link-color-light": "var(--ika-link-color-light)",
        "ika-ghost-white": "var(--ika-ghost-white)",
        "ika-ghost-dark": "var(--ika-ghost-dark)",
        "ifm-background-color-dark": "var(--ifm-background-color-dark)",
        "ika-white": "rgb(var(--ika-white)/<alpha-value>)",
        "ika-card-dark": "rgb(var(--ika-card-dark)/<alpha-value>)",
        "ika-card-darker": "rgb(var(--ika-card-darker)/<alpha-value>)",
      },
      borderRadius: {
        ika: "40px",
      },
      boxShadow: {
        ika: "0px 0px 4px rgba(0, 0, 0, 0.02)",
        "ika-button": "0px 1px 2px rgba(16, 24, 40, 0.05)",
        "ika-notification": "0px 0px 20px rgba(29, 55, 87, 0.11)",
      },
      gradientColorStopPositions: {
        36: "36%",
      },
    },
  },
  plugins: [],
};
