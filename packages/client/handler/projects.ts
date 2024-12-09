import type { ApiClient } from "@/service/mod.ts";
import type { CreateProjectRequest, CreateProjectResponse, ProjectPathParams } from "@/typing/project.ts"

export interface Project {
	/** Creates a new Project. */
	createNewProject(param: ProjectPathParams, body: CreateProjectRequest): Promise<CreateProjectResponse>;

	// modifyExistingProject()
}

export class ProjectService implements Project {
	readonly #internal: ApiClient;

	constructor(internal: ApiClient) {
		this.#internal = internal;
	}

	async createNewProject(param: ProjectPathParams, body: CreateProjectRequest): Promise<CreateProjectResponse> {
		return await this.#internal.makeFetch({
			path: `accounts/${param.account}/projects`,
			method: "post",
			body
		});
	}
}
