export interface RequestOptions extends Omit<RequestInit, 'body'> {
    params?: Record<string, string | number | boolean | undefined | null>;
    headers?: Record<string, string>;
}

/**
 * Thrown by the http utility for any non-2xx response.
 * Carry the parsed JSON body so callers (e.g. formErrorsMixin.applyApiErrors)
 * can inspect structured error payloads like:
 *   { status: false, errors: [{ key: "name", message: "validation_required" }] }
 */
export class HttpError extends Error {
    /** HTTP status code (e.g. 400, 401, 422, 500) */
    status: number;
    /** Parsed JSON body of the error response, or null for non-JSON responses */
    data: any;

    constructor(message: string, status: number, data: any = null) {
        super(message);
        this.name = 'HttpError';
        this.status = status;
        this.data = data;
    }
}

function buildUrl(url: string, params?: Record<string, string | number | boolean | undefined | null>): string {
    if (!params) return url;

    const searchParams = new URLSearchParams();
    for (const [key, value] of Object.entries(params)) {
        if (value !== undefined && value !== null) {
            searchParams.append(key, String(value));
        }
    }

    const queryString = searchParams.toString();
    if (!queryString) return url;

    return url.includes('?') ? `${url}&${queryString}` : `${url}?${queryString}`;
}

/**
 * Core fetch wrapper.
 *
 * - Returns the parsed response body `T` on success (2xx).
 * - Throws `HttpError` on any non-2xx HTTP response, with `err.status` and
 *   `err.data` set to the parsed response body.
 * - Throws a plain `Error` on network failures (no response at all).
 *
 * This means callers never need to check `res.ok`; all error paths go through
 * the catch block, which is consistent with the native Fetch API convention.
 */
async function request<T = any>(
    url: string,
    method: 'GET' | 'POST' | 'PUT' | 'PATCH' | 'DELETE',
    body?: any,
    options: RequestOptions = {}
): Promise<T> {
    const { params, headers = {}, ...customConfig } = options;

    const defaultHeaders: Record<string, string> = {
        'Accept': 'application/json',
    };

    if (typeof window !== 'undefined' && window.localStorage) {
        let token = localStorage.getItem('auth_token');
        if (token) {
            // gloo_storage / serde might store JSON strings (e.g. "\"xyz\"")
            try {
                const parsed = JSON.parse(token);
                if (typeof parsed === 'string') {
                    token = parsed;
                }
            } catch {
                // Not JSON string encoded, use as-is
            }
            if (token) {
                defaultHeaders['Authorization'] = `Bearer ${token}`;
            }
        }
    }

    let requestBody: any = undefined;

    if (body !== undefined && body !== null) {
        if (body instanceof FormData || body instanceof URLSearchParams || body instanceof Blob) {
            requestBody = body;
        } else {
            defaultHeaders['Content-Type'] = 'application/json';
            requestBody = JSON.stringify(body);
        }
    }

    const config: RequestInit = {
        method,
        headers: {
            ...defaultHeaders,
            ...headers,
        },
        body: requestBody,
        ...customConfig,
    };

    const finalUrl = buildUrl(url, params);

    const response = await fetch(finalUrl, config);
    const isJson = response.headers.get('content-type')?.includes('application/json');
    const data = isJson ? await response.json() : await response.text();

    if (!response.ok) {
        // Prefer a `message` field in the JSON body, fall back to HTTP statusText
        const errorMessage =
            (typeof data === 'object' && data?.message) ||
            response.statusText ||
            'Request failed';
        throw new HttpError(errorMessage, response.status, data);
    }

    return data as T;
}

export const http = {
    get: <T = any>(url: string, options?: RequestOptions) =>
        request<T>(url, 'GET', undefined, options),

    post: <T = any>(url: string, body?: any, options?: RequestOptions) =>
        request<T>(url, 'POST', body, options),

    put: <T = any>(url: string, body?: any, options?: RequestOptions) =>
        request<T>(url, 'PUT', body, options),

    patch: <T = any>(url: string, body?: any, options?: RequestOptions) =>
        request<T>(url, 'PATCH', body, options),

    delete: <T = any>(url: string, options?: RequestOptions) =>
        request<T>(url, 'DELETE', undefined, options),
};

export default http;
