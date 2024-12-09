import type { ApiClient } from "@/service/mod.ts";

export interface Workflow {}

export class WorkflowService implements Workflow {
	readonly #internal: ApiClient;

	constructor(internal: ApiClient) {
		this.#internal = internal;
	}
}
