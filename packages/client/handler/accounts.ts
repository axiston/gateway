import type { ApiClient } from "@/service/mod.ts";

export interface Account {}

export class AccountService implements Account {
	readonly #internal: ApiClient;

	constructor(internal: ApiClient) {
		this.#internal = internal;
	}
}
