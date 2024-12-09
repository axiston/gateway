import type { ApiClient } from "@/service/mod.ts";

export interface Runtime {}

export class RuntimeService implements Runtime {
	readonly #internal: ApiClient;

	constructor(internal: ApiClient) {
		this.#internal = internal;
	}
}
