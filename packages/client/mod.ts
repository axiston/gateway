import { ApiClientService } from "@/service/mod.ts";
import type { Account, Project, Runtime, Workflow } from "@/handler/mod.ts";
export type {
	Account,
	Project,
	Runtime,
	Workflow,
	Websocket,
} from "@/handler/mod.ts";
import {
	AccountService,
	ProjectService,
	RuntimeService,
	WorkflowService,
} from "@/handler/mod.ts";

/**
 * {@link Client} connection options.
 */
export interface ClientOptions {
	/**
	 * TODO.
	 */
	readonly apiToken?: string;

	/**
	 * TODO.
	 */
	readonly userAgent?: string;

	/**
	 * Overrides the default base url.
	 *
	 * Default value: https://axiston.com/
	 */
	readonly baseUrl?: string | URL;
}

/**
 * A complete axiston.com gateway client.
 *
 * @link https://axiston.com/
 */
export class Client {
	readonly #account: AccountService;
	readonly #project: ProjectService;
	readonly #runtime: RuntimeService;
	readonly #workflow: WorkflowService;

	/**
	 * Returns a new {@link Client}.
	 *
	 * @param options connection options.
	 */
	constructor(options?: ClientOptions) {
		const apiClient = new ApiClientService(options);
		this.#account = new AccountService(apiClient);
		this.#project = new ProjectService(apiClient);
		this.#runtime = new RuntimeService(apiClient);
		this.#workflow = new WorkflowService(apiClient);
	}

	/** axiston.com/:account */
	get accounts(): Account {
		return this.#account;
	}

	/** axiston.com/:account/:runtime */
	get runtimes(): Runtime {
		return this.#runtime;
	}

	/** axiston.com/:account/:project */
	get projects(): Project {
		return this.#project;
	}

	/** axiston.com/:account/:project/:workflow */
	get workflows(): Workflow {
		return this.#workflow;
	}
}
