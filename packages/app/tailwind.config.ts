import type { Config } from "tailwindcss";
import theme from "tailwindcss/defaultTheme";
// import plugin from 'tailwindcss/plugin'

export default {
	content: [
		"./components/**/*.{html,md,mdx,astro,svelte}",
		"./content/**/*.{html,md,mdx,astro,svelte}",
		"./layouts/**/*.{html,md,mdx,astro,svelte}",
		"./pages/**/*.{html,md,mdx,astro,svelte}",
	],
	darkMode: "class",
	theme: {
		extend: {
			fontFamily: {
				sans: ["Barlow", ...theme.fontFamily.sans],
				mono: ["Jetbrains", ...theme.fontFamily.mono],
			},
		},
	},
} satisfies Config;
