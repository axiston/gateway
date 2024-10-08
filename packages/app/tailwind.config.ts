import type { Config } from "tailwindcss";
// import plugin from 'tailwindcss/plugin'

export default {
	content: [
		"./components/**/*.{html,md,mdx,astro,svelte}",
		"./content/**/*.{html,md,mdx,astro,svelte}",
		"./layouts/**/*.{html,md,mdx,astro,svelte}",
		"./pages/**/*.{html,md,mdx,astro,svelte}",
	],
	darkMode: "class",
} satisfies Config;
