import type { ApiClient } from "@/service/client.ts";

export interface Websocket {}

export class WebsocketService implements Websocket {
	readonly #internal: ApiClient;

	constructor(internal: ApiClient) {
		this.#internal = internal;
	}
}
