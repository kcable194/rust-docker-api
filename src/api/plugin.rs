//! Install, create and manage plugins

use crate::{
    conn::{Headers, Payload},
    models,
    opts::PluginListOpts,
    Result,
};
use containers_api::url::{construct_ep, encoded_pair};

use std::path::Path;

impl_api_ty!(Plugin => name);

impl Plugin {
    impl_api_ep! {plug: Plugin, resp
        Inspect -> &format!("/plugins/{}/json", plug.name), rs_docker_api_stubs::models::Plugin
        ForceDelete -> &format!("/plugins/{}", plug.name), rs_docker_api_stubs::models::Plugin
    }

    api_doc! { Plugin => Enable
    |
    /// Enable a plugin.
    pub async fn enable(&self, timeout: Option<u64>) -> Result<()> {
        let query = timeout.map(|timeout| encoded_pair("timeout", timeout));
        self.docker
            .post(
                &construct_ep(format!("/plugins/{}/enable", self.name), query),
                Payload::empty(),
                Headers::none()
            )
            .await
            .map(|_| ())
    }}

    api_doc! { Plugin => Disable
    |
    /// Disable a plugin.
    pub async fn disable(&self) -> Result<()> {
        self.docker
            .post(&format!("/plugins/{}/disable", self.name), Payload::empty(), Headers::none())
            .await
            .map(|_| ())
    }}

    api_doc! { Plugin => Push
    |
    /// Push a plugin to the registry.
    pub async fn push(&self) -> Result<()> {
        self.docker
            .post(&format!("/plugins/{}/push", self.name), Payload::empty(), Headers::none())
            .await
            .map(|_| ())
    }}

    api_doc! { Plugin => Create
    |
    /// Create a plugin from a tar archive on the file system. The `path` parameter is a path
    /// to the tar containing plugin rootfs and manifest.
    pub async fn create<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        self.docker
            .post(
                &format!("/plugins/{}/create", self.name),
                Payload::Text(path.as_ref().to_string_lossy().to_string()),
                Headers::none()
            )
            .await
            .map(|_| ())
    }}
}

impl Plugins {
    impl_api_ep! {plug: Plugin, resp
        List -> "/plugins", rs_docker_api_stubs::models::Plugin
    }
}
