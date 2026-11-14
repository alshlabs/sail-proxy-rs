pub fn match_alias(input: &str) -> &str {
  match input {
    "deepseek-flash" | "deepseek-flash-balanced" => "deepseek-ai/DeepSeek-V4.1-Flash-1017",
    "deepseek-flash-flex" => "deepseek-ai/DeepSeek-V4.1-Flash-1017",
    v => v,
  }
}
