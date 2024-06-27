use anyhow::{anyhow, Result};
use pdk::hl::*;
use pdk::logger;

use crate::generated::config::Config;

// Copyright 2023 Salesforce, Inc. All rights reserved.
mod generated;

// This filter shows how to log a specific request header.
// You can extend the function and use the configurations exposed in config.rs file
async fn request_filter(request_state: RequestState, config: &Config) -> Flow<()> {
    let headers_state = request_state.into_headers_state().await;
    let vec = &config.all;
    for condition in vec {
        let actual_header_value = &headers_state.handler().header(condition.header_name.as_str());
        match actual_header_value {
            None => {
                logger::info!("Expecting header: `{}` to be: `{}` but was not present", condition.header_name, condition.header_value);
                return Flow::Break(Response::new(403));
            }
            Some(actual_header_value) => {
                if !actual_header_value.eq(&condition.header_value) {
                    logger::info!("Expecting header: `{}` to be: `{}` but was: `{:?}`", condition.header_name, condition.header_value, actual_header_value);
                    return Flow::Break(Response::new(403));
                }
            }
        }
    }
    Flow::Continue(())
}

#[entrypoint]
async fn configure(launcher: Launcher, Configuration(bytes): Configuration) -> Result<()> {
    let config: Config = serde_json::from_slice(&bytes).map_err(|err| {
        anyhow!(
            "Failed to parse configuration '{}'. Cause: {}",
            String::from_utf8_lossy(&bytes),
            err
        )
    })?;
    let filter = on_request(|rs| request_filter(rs, &config));
    launcher.launch(filter).await?;
    Ok(())
}
