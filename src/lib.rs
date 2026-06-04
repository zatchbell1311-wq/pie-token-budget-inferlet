//! Token Budget Inferlet - by Dhruv Dubey
use inferlet::{Context, Result, chat, model::Model, runtime, sample::Sampler};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Input {
    prompt: String,
    #[serde(default = "default_budget")]
    token_budget: usize,
    #[serde(default = "default_system")]
    system: String,
    #[serde(default = "default_temperature")]
    temperature: f32,
}

#[derive(Serialize)]
struct Output {
    text: String,
    tokens_used: usize,
    token_budget: usize,
    utilization_pct: f32,
    budget_exhausted: bool,
}

fn default_budget() -> usize { 128 }
fn default_system() -> String { "You are a helpful, concise assistant. Be brief.".into() }
fn default_temperature() -> f32 { 0.6 }

#[inferlet::main]
async fn main(input: Input) -> Result<String> {
    let models = runtime::models();
    let model_name = models.first().ok_or("No models available")?;
    let model = Model::load(model_name)?;
    let mut ctx = Context::new(&model)?;
    ctx.system(&input.system).user(&input.prompt).cue();
    let mut chat = chat::Decoder::new(&model);
    let mut text = String::new();
    let mut tokens_used: usize = 0;
    let mut budget_exhausted = false;
    let mut g = ctx
        .generate(Sampler::TopP { temperature: input.temperature, p: 0.95 })
        .max_tokens(input.token_budget)
        .stop(&chat::stop_tokens(&model));
    while let Some(step) = g.next()? {
        let out = step.execute().await?;
        if out.tokens.is_empty() { continue; }
        tokens_used += out.tokens.len();
        if tokens_used >= input.token_budget {
            budget_exhausted = true;
            match chat.feed(&out.tokens)? {
                chat::Event::Delta(s) => text.push_str(&s),
                chat::Event::Done(s) => text = s,
                _ => {}
            }
            break;
        }
        match chat.feed(&out.tokens)? {
            chat::Event::Delta(s) => { print!("{}", s); text.push_str(&s); }
            chat::Event::Done(s) => { text = s; break; }
            _ => {}
        }
    }
    let utilization_pct = (tokens_used as f32 / input.token_budget as f32) * 100.0;
    let output = Output { text, tokens_used, token_budget: input.token_budget, utilization_pct, budget_exhausted };
    Ok(serde_json::to_string_pretty(&output).unwrap_or_else(|_| "serialization error".into()))
}

