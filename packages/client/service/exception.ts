/**
 * TODO.
 */
export class ClientError extends Error {
	/**
	 * Returns a new {@link ClientError}.
	 */
	constructor(name: string, message: string) {
		super(message);
		this.name = name;
	}
}

/**
 * Error that occurred during {@link fetch} or {@link Response.json}.
 *
 * @see ClientError
 */
export class ClientHttpError extends ClientError {
	/**
	 * Returns a new {@link ClientHttpError}.
	 */
	constructor(name: string, message: string, statusCode: number) {
		super(name, message);
	}

	/**
	 * Returns a new {@link ClientHttpError} from the JSON response.
	 *
	 * @param errorResponse
	 * @param statusCode
	 */
	static buildFromJson(
		errorResponse: unknown,
		statusCode: number,
	): ClientHttpError {
		// TODO.
		return new ClientHttpError("", "", statusCode);
	}

	/**
	 * Returns a new {@link ClientHttpError} from the thrown exception.
	 *
	 * @param thrownError
	 * @param statusCode
	 */
	static buildFromError(
		thrownError: Error,
		statusCode: number,
	): ClientHttpError {
		return new ClientHttpError(
			thrownError.name,
			thrownError.message,
			statusCode,
		);
	}

	/**
	 * Returns a new {@link ClientHttpError} from the status code only.
	 *
	 * @param statusCode
	 */
	static buildFromStatus(statusCode: number): ClientHttpError {
		return new ClientHttpError("HttpError", "", statusCode);
	}
}

/**
 * Error that occurred during {@link JSON.stringify}.
 *
 * @see ClientError
 */
export class ClientJsonError extends ClientError {
	/**
	 * Returns a new {@link ClientJsonError}.
	 */
	constructor(name: string, message: string) {
		super(name, message);
	}

	/**
	 * Returns a new {@link ClientJsonError} from the thrown exception.
	 *
	 * @param thrownError
	 */
	static buildFromError(thrownError: Error): ClientJsonError {
		return new ClientJsonError(thrownError.name, thrownError.message);
	}

	/**
	 * Returns a new {@link ClientJsonError}.
	 */
	static buildAsDefault(): ClientJsonError {
		return new ClientJsonError(
			"JsonError",
			"The request body is not convertable into JSON format.",
		);
	}
}
