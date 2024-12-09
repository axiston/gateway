import type { ClientOptions } from "@/mod.ts";
import {
	ClientHttpError,
	ClientJsonError,
	type Method,
} from "@/service/mod.ts";

export interface ApiClient {
	makeFetch<T, U>(options: ApiClientOptions<T>): Promise<U>;
}

export interface ApiClientOptions<T = unknown> {
	path: string;
	method: Method;
	body: T;
}

export class ApiClientService implements ApiClient {
	readonly #options: Required<ClientOptions>;

	/**
	 * Returns a new {@link ApiClientService}.
	 *
	 * @param options connection options.
	 */
	constructor(options?: ClientOptions) {
		this.#options = Object.assign({}, options, {
			apiToken: "none",
			userAgent: navigator.userAgent,
			baseUrl: new URL("https://axiston.com/"),
		} satisfies Required<ClientOptions>);
	}

	/**
	 * - makes the {@link Request} with provided options.
	 * - returns the {@link Response} as a JSON.
	 *
	 * @throws ClientHttpError
	 * @throws ClientJsonError
	 *
	 * @param options
	 */
	async makeFetch<T, U>(options: ApiClientOptions<T>): Promise<U> {
		let body: string;
		try {
			body = JSON.stringify(options.body);
		} catch (error: unknown) {
			if (error instanceof Error) {
				throw ClientJsonError.buildFromError(error);
			} else {
				throw ClientJsonError.buildAsDefault();
			}
		}

		const requestInput = new URL(options.path, this.#options.baseUrl);
		const headers = new Headers({
			Authorization: `Bearer ${this.#options.apiToken}`,
			"User-Agent": this.#options.userAgent,
			"Content-Type": "application/json",
		});

		const requestOptions = { method: options.method, headers, body };
		const response = await fetch(requestInput, requestOptions);

		let responseJson: U;
		try {
			// TODO: Optional JSON parsing.
			responseJson = await response.json();
		} catch (error: unknown) {
			if (error instanceof Error) {
				throw ClientHttpError.buildFromError(error, response.status);
			} else {
				throw ClientHttpError.buildFromStatus(response.status);
			}
		}

		if (!response.ok) {
			throw ClientHttpError.buildFromJson(responseJson, response.status);
		}

		return responseJson;
	}
}
