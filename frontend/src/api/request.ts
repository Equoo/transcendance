import { APIError, type ProblemDetail } from "@/api/problem_detail";
import callApi from "@/tokens/callApi";

type HttpMethod = "GET" | "POST" | "PUT" | "PATCH" | "DELETE";

export async function responseError(response: Response): Promise<Error> {
	const text = await response.text().catch(() => "");
	let problem: Partial<ProblemDetail> | null = null;

	try {
		if (text) {
			problem = JSON.parse(text) as Partial<ProblemDetail>;
		}
	} catch {
		// empty catch
	}

	if (response.status >= 500 || problem === null || typeof problem !== "object") {
		return new Error(
			problem?.detail ?? problem?.title ?? response.statusText,
		);
	}

	return new APIError({
		type: "",
		title: response.statusText,
		traceId: "",
		errorCode: "",
		...problem,
		status: response.status,
	});
}

export async function request(
	url: string,
	method: HttpMethod = "GET",
	body?: unknown,
): Promise<Response> {
	const init: RequestInit = { method };
	if (typeof body !== "undefined") {
		init.headers = { "Content-Type": "application/json" };
		init.body = JSON.stringify(body);
	}

	const response = await callApi(url, init);
	if (!response.ok) {
		throw await responseError(response);
	}

	return response;
}

export async function requestJson<T>(
	url: string,
	method?: HttpMethod,
	body?: unknown,
): Promise<T> {
	const response = await request(url, method, body);
	return (await response.json()) as T;
}
