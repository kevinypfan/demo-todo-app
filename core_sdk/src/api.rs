use anyhow::{anyhow, Result};
use log::info;
use ureq::{Agent, AgentBuilder, Response};

#[derive(Debug, Clone)]
pub struct ApiClient {
  agent: Agent,
  base_url: String,
}

impl ApiClient {
  pub fn new(base_url: String) -> Self {
    let agent = AgentBuilder::new().build();
    Self { agent, base_url }
  }

  pub fn get(&self, path: &str, query_params: Option<String>) -> Result<Response> {
    let mut full_url = format!("{}{}", self.base_url, path);
    if let Some(query_params) = query_params {
      full_url = format!("{}?{}", full_url, query_params);
    }

    self
      .agent
      .get(&full_url)
      .call()
      .map_err(|err| anyhow!("{:?}", err))
  }

  pub fn post(
    &self,
    path: &str,
    query_params: Option<String>,
    body: Option<String>,
  ) -> Result<Response> {
    let mut full_url = format!("{}{}", self.base_url, path);
    if let Some(query_params) = query_params {
      full_url = format!("{}?{}", full_url, query_params);
    }
    info!("full_url={:?}", full_url);
    info!("body_string={:?}", body);

    let client = self
      .agent
      .post(&full_url)
      .set("Content-Type", "application/json");

    if let Some(body) = body {
      client
        .send_string(&body)
        .map_err(|err| anyhow!("{:?}", err))
    } else {
      client.call().map_err(|err| anyhow!("{:?}", err))
    }
  }

  pub fn put(
    &self,
    path: &str,
    query_params: Option<String>,
    body: Option<String>,
  ) -> Result<Response> {
    let mut full_url = format!("{}{}", self.base_url, path);
    if let Some(query_params) = query_params {
      full_url = format!("{}?{}", full_url, query_params);
    }
    info!("full_url={:?}", full_url);
    info!("body_string={:?}", body);

    let client = self
      .agent
      .put(&full_url)
      .set("Content-Type", "application/json");

    if let Some(body) = body {
      client
        .send_string(&body)
        .map_err(|err| anyhow!("{:?}", err))
    } else {
      client.call().map_err(|err| anyhow!("{:?}", err))
    }
  }

  pub fn delete(&self, path: &str, query_params: Option<String>) -> Result<Response> {
    let mut full_url = format!("{}{}", self.base_url, path);
    if let Some(query_params) = query_params {
      full_url = format!("{}?{}", full_url, query_params);
    }

    self
      .agent
      .delete(&full_url)
      .call()
      .map_err(|err| anyhow!("{:?}", err))
  }
}
