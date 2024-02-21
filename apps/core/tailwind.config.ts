// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type Config } from 'tailwindcss';
import colors from 'tailwindcss/colors';
import { fontFamily } from 'tailwindcss/defaultTheme';

/** The minimum line height that text should use to avoid clipping and overflow scrolling */
const MIN_LINE_HEIGHT = '1.13';

export default {
	content: ['./src/**/*.{js,jsx,ts,tsx}'],
	theme: {
		// Overwrite colors to avoid accidental usage of Tailwind colors:
		colors: {
			white: colors.white,
			black: colors.black,
			transparent: colors.transparent,
			inherit: colors.inherit,

			gray: {
				100: '#182435',
				95: '#2A3645',
				90: '#383F47',
				85: '#5A6573',
				80: '#636870',
				75: '#767A81',
				70: '#898D93',
				65: '#9C9FA4',
				60: '#C3C5C8',
				55: '#D7D8DA',
				50: '#E9EAEB',
				45: '#E3E6E8',
				40: '#F3F6F8',
				35: '#FEFEFE',
			},

			sui: {
				DEFAULT: '#333533',
				bright: '#E9EAEB',
				light: '#ababab',
				primaryBlue2023: '#EFF230',
				lightest: '#F1F8FD',
				dark: '#202020',
			},

			steel: {
				DEFAULT: '#D6D6D6',
				dark: '#b8b6b6',
				darker: '#202020',
			},

			issue: {
				DEFAULT: '#f05836',
				dark: '#e63912',
				light: '#FFECE6',
			},
			hero: {
				DEFAULT: '#828282',
				dark: '#333533',
				darkest: '#202020',
			},
			success: {
				DEFAULT: '#EFF230',
				dark: '#edcb1c',
				light: '#f7f2d5',
			},
			warning: {
				DEFAULT: '#f28e24',
				dark: '#8D6E15',
				light: '#e9b075',
			},
			headerNav: '#20201e',
			search: {
				fill: '#202020',
			},
			offwhite: '#202020',
			offblack: '#111111',
			ebony: '#060501',
			avocado: {
				200: '#b5b8b5',
			},
		},

		extend: {
			scale: {
				'101': '1.01',
			},
			// backdrop-blur values that are found in the design:
			backdropBlur: {
				sm: '8px',
				md: '16px',
				DEFAULT: '20px',
				lg: '24px',
				xl: '32px',
			},
			colors: {
				'gradient-blue-start': '#EFF230',
				'gradient-blue-end': '#FFD100',
				'gradients-graph-cards-start': '#202020',
				'gradients-failure-start': '#333533',
			},
			// Line-heights that are found in the design:
			lineHeight: {
				80: '0.8',
				100: '1',
				130: '1.3',
			},
			boxShadow: {
				xs: '0px 1px 2px rgba(16, 24, 40, 0.05)',
				sm: '0px 1px 2px 0px rgba(86, 104, 115, 0.08)',
				md: '1px 2px 8px 2px rgba(86, 104, 115, 0.06)',
				lg: '0px 0px 44px 0px rgba(86, 104, 115, 0.22)',
				DEFAULT: '0px 0px 20px 0px rgba(86, 104, 115, 0.14)',
				notification: '0px 0px 20px rgba(29, 55, 87, 0.11)',
				moduleOption:
					'0px 0px 29px rgba(0, 0, 0, 0.13), 0px 0px 44px rgba(0, 0, 0, 0.15), 0px 0px 0px rgba(0, 0, 0, 0.2)',
				blurXl: '0 0 20px 0 rgba(0, 0, 0, 0.3)',
				button: '0px 1px 2px rgba(16, 24, 40, 0.05)',
				glow: '0 0px 6px 4px rgba(213,247,238,1)',
				drop: '0px 0px 10px rgba(111, 188, 240, 0.2)',
				'effect-ui-regular':
					'0px 5px 30px 0px rgba(86, 104, 115, 0.20), 0px 0px 0px 1px rgba(86, 104, 115, 0.03)',
				panel: '0px 2px 7px 0px rgba(160, 182, 195, 0.32)',
				dropdownContent: '0px 1px 2px 0px rgba(21, 82, 123, 0.08)',
				'effect-ui-wallet-content': '0px -5px 20px 5px rgba(111, 188, 240, 0.11)',
			},
			fontSize: {
				// Text sizes:
				body: ['14px', MIN_LINE_HEIGHT],
				bodySmall: ['13px', MIN_LINE_HEIGHT],
				subtitle: ['12px', MIN_LINE_HEIGHT],
				subtitleSmall: ['11px', MIN_LINE_HEIGHT],
				subtitleSmallExtra: ['10px', MIN_LINE_HEIGHT],
				caption: ['12px', MIN_LINE_HEIGHT],
				captionSmall: ['11px', MIN_LINE_HEIGHT],
				captionSmallExtra: ['10px', MIN_LINE_HEIGHT],
				iconTextLarge: ['48px', MIN_LINE_HEIGHT],

				// Heading sizes:
				heading1: ['28px', MIN_LINE_HEIGHT],
				heading2: ['24px', MIN_LINE_HEIGHT],
				heading3: ['20px', MIN_LINE_HEIGHT],
				heading4: ['18px', MIN_LINE_HEIGHT],
				heading5: ['17px', MIN_LINE_HEIGHT],
				heading6: ['16px', MIN_LINE_HEIGHT],

				// Paragraph sizes:
				pHeading6: ['16px', '1.4'],
				pBody: ['14px', '1.4'],
				pBodySmall: ['13px', '1.4'],
				pSubtitle: ['12px', '1.4'],
				pSubtitleSmall: ['11px', '1.4'],
			},
			fontFamily: {
				system: fontFamily.sans,
				sans: ['Inter Variable', 'Inter', ...fontFamily.sans],
				mono: ['Red Hat Mono Variable', 'Red Hat Mono', ...fontFamily.mono],
			},
			keyframes: {
				shimmer: {
					'0%': { 'background-position': '-1000px 0' },
					'100%': { 'background-position': '1000px 0' },
				},
			},
			animation: {
				shimmer: 'shimmer 2s infinite linear',
			},
			spacing: {
				1.25: '0.3125rem',
				3.75: '0.9375rem',
				4.5: '1.125rem',
				7.5: '1.875rem',
				17: '4.25rem',
				18: '4.5rem',
				19: '4.75rem',
				50: '12.5rem',
				verticalListShort: '13.0625rem',
				verticalListLong: '35.6875rem',
				600: '37.5rem',
				header: '68px',
			},
			height: {
				12.5: '3.125rem',
				31.5: '7.5rem',
				100: '25rem',
				120: '30rem',
				300: '75rem',
				coinsAndAssetsContainer: '31.25rem',
			},
			maxHeight: {
				coinsAndAssetsContainer: '31.25rem',
				ownCoinsPanel: '14.375rem',
			},
			width: {
				12.5: '3.125rem',
				31.5: '7.5rem',
				walletLogo: '4.813rem',
			},
			maxWidth: {
				80: '20rem',
			},
			minWidth: {
				10: '2.5rem',
				18: '4.5rem',
				44: '11rem',
				50: '12.5rem',
				transactionColumn: '31.875rem',
				smallThumbNailsViewContainer: '13.125rem',
				smallThumbNailsViewContainerMobile: '9.375rem',
				coinItemContainer: '15.625rem',
			},
			minHeight: {
				14: '3.5rem',
			},
			transitionTimingFunction: {
				'ease-in-out-cubic': 'cubic-bezier(0.65, 0, 0.35, 1)',
				'ease-out-cubic': 'cubic-bezier(0.33, 1, 0.68, 1)',
			},
			transitionDuration: {
				400: '400ms',
			},
			backgroundImage: {
				placeholderGradient01: 'linear-gradient(165.96deg, #333533 10%, #202020 95%)',
				placeholderShimmer:
					'linear-gradient(90deg, #333533 -24.18%, rgba(237 242 245 / 40%) 73.61%, #202020 114.81%, #202020 114.82%)',
				'gradients-graph-cards': 'linear-gradient(180deg, #333533 51.68%, #202020 100%)',
				'gradients-failure': 'linear-gradient(166deg, #333533 0%, #202020 100%)',
				objectCard: 'linear-gradient(166deg, #333533 9.97%, #202020 94.97%)',
			},
			rotate: {
				135: '135deg',
			},
			borderRadius: {
				'2lg': '0.625rem',
			},
			aspectRatio: {
				square: '1 / 1',
			},
		},
	},
	corePlugins: {
		aspectRatio: true,
	},
	plugins: [
		require('@headlessui/tailwindcss'),
		require('@tailwindcss/aspect-ratio'),
		require('@tailwindcss/forms')({
			strategy: 'class',
		}),
	],
} satisfies Config;
