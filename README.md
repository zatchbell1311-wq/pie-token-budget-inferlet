# Token Budget Inferlet

A Pie inferlet that enforces hard token budgets during LLM generation. Built as part of learning the Pie programmable inference engine architecture.

## What it does

Standard LLM serving gives you no control over generation once it starts. This inferlet runs inside the Pie engine and tracks tokens consumed in real time, stopping generation the moment a specified budget is exhausted. It returns the generated text along with a utilization report.

## Output format

```json
{
  "text": "...",
  "tokens_used": 80,
  "token_budget": 80,
  "utilization_pct": 100.0,
  "budget_exhausted": true
}
```

## Why this matters

Token usage directly maps to latency and cost in production LLM systems. Enforcing hard budgets from inside the engine — with direct access to the generation loop — is only possible with Pie's inferlet architecture. Traditional serving systems like vLLM or SGLang have no mechanism for this kind of in-engine control.

## Build

```bash
cargo build --release --target wasm32-wasip2
```

Requires Rust with the wasm32-wasip2 target:

```bash
rustup target add wasm32-wasip2
```

## Run

```bash
pie run --path target/wasm32-wasip2/release/my_first_inferlet.wasm --manifest Pie.toml --stdout -- --prompt "Your prompt here" --token-budget 80
```

## Input parameters

- `prompt` — the user prompt
- `token_budget` — maximum tokens to generate (default: 128)
- `system` — system message (default: "You are a helpful, concise assistant. Be brief.")
- `temperature` — sampling temperature (default: 0.6)

## Background

This project connects to research on token-efficient LLM memory architectures (DSPM). The token budget enforcement pattern demonstrated here is a building block for larger systems that need fine-grained control over inference resource consumption.



DEMO:
<img width="1919" height="989" alt="image" src="https://github.com/user-attachments/assets/f962dd62-bad7-40ad-a322-5e7ca747a125" />

