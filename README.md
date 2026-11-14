# sail-proxy-rs

An OpenAI Chat Completions proxy for [Sail Research](https://docs.sailresearch.com) `flex` mode.

Sail's flex window only runs as a background job on the Responses API, which existing tooling can't use. This proxy accepts a normal `/v1/chat/completions` request, submits it to Sail as a flex background job, polls until it finishes, and returns the result, streaming it back when requested. Tool calls are supported.

## Install

Prebuilt binaries for Linux (`amd64`, `arm64`) are published on [GitHub Releases](https://github.com/alshdavid/sail-proxy-rs/releases).

```sh
curl -fsSL https://github.com/alshdavid/sail-proxy-rs/releases/latest/download/sail-proxy-rs-linux-amd64.tar.gz | tar -xz
./sail-proxy-rs
```

Or build from source with `cargo run -- serve`.

## Run

```sh
sail-proxy-rs serve
```

| Variable | Default | |
|---|---|---|
| `SAIL_API_BASE` | `https://api.sailresearch.com/v1` | Sail API endpoint. |
| `SAIL_PROXY_ADDRESS` | `0.0.0.0:9090` | Address the proxy listens on. |

The proxy forwards each client's `Authorization: Bearer <token>` header to Sail, so clients authenticate with their own Sail API key. Requests without a bearer token are rejected with `401`.

## Update

On startup the proxy checks GitHub Releases and prints a notice when a newer version is available. To update in place:

```sh
sail-proxy-rs update
```

## Use

Point any OpenAI-compatible client at `http://localhost:9090/v1`. For example, in opencode:

```json
{
  "provider": {
    "sail": {
      "npm": "@ai-sdk/openai-compatible",
      "options": { "baseURL": "http://localhost:9090/v1" },
      "models": {
        "deepseek-flash": { "id": "deepseek-ai/DeepSeek-V4.1-Flash" }
      }
    }
  }
}
```

## Limitations

- The response arrives in one go once the job finishes; keep-alives are sent while waiting.
- `response_format: json_object` and non-function tools are not supported by Sail.
