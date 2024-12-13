import { defineConfig } from "astro/config";

// Official integrations.
import svelte from "@astrojs/svelte";
import tailwind from "@astrojs/tailwind";
import mdx from "@astrojs/mdx";
import node from "@astrojs/node";

// Third-party integrations.
import icon from "astro-icon";

// TODO: Replace Eslint & Prettier with Biome.
// BLOCKER: https://github.com/biomejs/biome/discussions/136
// BLOCKER: https://github.com/biomejs/biome/discussions/1254

// TODO: Use svelte for dynamic islands.
// TODO: Evaluate roxiness/routify for routing.
// See: https://github.com/roxiness/routify

// https://astro.build/config
export default defineConfig({
	site: "https://axiston.com",
	srcDir: ".",
	redirects: {
		"/": "/projects",
	},
	integrations: [
		svelte(),
		tailwind(),
		mdx(),
		icon({
			iconDir: "./assets",
		}),
	],
	output: "server",
	adapter: node({
		mode: "standalone",
	}),
});
