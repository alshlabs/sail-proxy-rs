use anyhow::Context;
use uhttp::Request;

pub fn access_token(req: &Request) -> anyhow::Result<String> {
  let header = req
    .headers
    .get(uhttp::headers::AUTHORIZATION)
    .context("missing Authorization header")?
    .to_str()
    .context("invalid Authorization header")?;

  let token = match header.split_once(' ') {
    Some((scheme, token)) if scheme.eq_ignore_ascii_case("bearer") => token.trim(),
    Some(_) => anyhow::bail!("unsupported Authorization scheme"),
    None => header.trim(),
  };

  if token.is_empty() {
    anyhow::bail!("missing access token");
  }

  Ok(token.to_string())
}
