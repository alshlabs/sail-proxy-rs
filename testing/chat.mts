/** Minimal client for an OpenAI-compatible /chat/completions endpoint. */

interface ChatCompletionMessage {
    role: 'system' | 'user' | 'assistant';
    content: string;
}

interface ChatCompletionResponse {
    choices?: Array<{
        index: number;
        message?: ChatCompletionMessage;
        finish_reason?: string | null;
    }>;
    error?: {message?: string; type?: string; code?: string | null};
}

export interface ChatOptions {
    /** Sampling temperature. */
    temperature?: number;
    /** Optional system prompt prepended to the conversation. */
    system?: string;
    /** Abort the request after this many milliseconds (default: no timeout). */
    timeoutMs?: number;
    /** Extra headers merged into the request. */
    headers?: Record<string, string>;
}

/**
 * Send a single user message to an OpenAI-compatible chat completions API and
 * return the assistant's reply as plain text.
 *
 * @param host   Base URL including the API version, e.g. `http://localhost:5051/v1`.
 * @param api_key Bearer token; omit or pass `null` for servers without auth.
 * @param model   Model name to send with the request, e.g. `gpt-4o-mini`.
 * @param message The user message to send.
 */
export async function chat(
    host: string,
    api_key: string | null,
    model: string,
    message: string,
    options: ChatOptions = {},
): Promise<string> {
    const {
        temperature = 0,
        system,
        timeoutMs,
        headers = {},
    } = options;

    const url = `${host.replace(/\/+$/, '')}/chat/completions`;

    const messages: ChatCompletionMessage[] = [];
    if (system) messages.push({role: 'system', content: system});
    messages.push({role: 'user', content: message});

    const controller = timeoutMs ? new AbortController() : undefined;
    const timer = controller ? setTimeout(() => controller.abort(), timeoutMs) : undefined;

    let response: Response;
    try {
        response = await fetch(url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json',
                ...(api_key ? {Authorization: `Bearer ${api_key}`} : {}),
                ...headers,
            },
            body: JSON.stringify({model, messages, temperature, stream: false}),
            signal: controller?.signal,
        });
    } catch (error) {
        if (controller?.signal.aborted) {
            throw new Error(`Chat request to ${url} timed out after ${timeoutMs}ms`);
        }
        throw new Error(
            `Chat request to ${url} failed: ${error instanceof Error ? error.message : String(error)}`,
            {cause: error},
        );
    } finally {
        if (timer) clearTimeout(timer);
    }

    const body = await response.text();

    if (!response.ok) {
        let detail = body;
        try {
            const parsed = JSON.parse(body) as ChatCompletionResponse;
            detail = parsed.error?.message ?? body;
        } catch {
            /* body was not JSON; keep the raw text */
        }
        throw new Error(
            `Chat request to ${url} failed with ${response.status} ${response.statusText}: ${detail}`,
        );
    }

    let data: ChatCompletionResponse;
    try {
        data = JSON.parse(body) as ChatCompletionResponse;
    } catch (error) {
        throw new Error(`Chat endpoint ${url} returned invalid JSON: ${body.slice(0, 500)}`, {
            cause: error,
        });
    }

    const content = data.choices?.[0]?.message?.content;
    if (typeof content !== 'string') {
        throw new Error(
            `Chat endpoint ${url} returned no message content: ${body.slice(0, 500)}`,
        );
    }

    return content;
}
