use std::{
    fmt::{self, Display, Formatter},
    fs,
    path::{Path},
};

use jsonwebtoken::EncodingKey;
use log::info;
use octocrab::{
    models::{repos::ContentItems, AppId, InstallationId},
    Octocrab,
};

use crate::errors::Error;

pub struct GithubContext {
    pub server: Option<String>,
    pub app: Option<AppContext>,
}
struct AppContext {
    key: EncodingKey,
    app_id: AppId,
    installation_id: InstallationId,
}

impl GithubContext {
    pub fn new(
        server: Option<String>,
        app_id: Option<u64>,
        private_key: Option<String>,
        installation_id: Option<u64>,
    ) -> Result<Self, Error> {
        let app = private_key
            .map(|key| {
                if Path::new(&key).exists() {
                    fs::read(&key).map_err(Error::IO).and_then(|key| {
                        EncodingKey::from_rsa_pem(&key).map_err(Error::JsonWebToken)
                    })
                } else {
                    EncodingKey::from_rsa_pem(key.as_bytes()).map_err(Error::JsonWebToken)
                }
            })
            .zip(app_id.zip(installation_id))
            .map(|(private_key, (app_id, installation_id))| {
                private_key.map(|key| AppContext {
                    key,
                    app_id: AppId(app_id),
                    installation_id: InstallationId(installation_id),
                })
            })
            .transpose()?;
        Ok(Self { server, app })
    }
}
#[derive(Debug)]
pub struct Github {
    pub octocrab: Octocrab,
}
#[derive(Debug,Clone)]
pub struct Repository {
    pub owner: String,
    pub name: String,
}
impl Display for Repository {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "owner {} repository {}", self.owner, self.name)
    }
}

impl Github {
    pub fn new(ctx: Option<GithubContext>) -> Result<Self, Error> {
        let instance = match &ctx {
            Some(ctx) => {
                info!("Create instance based on context");
                let builder = octocrab::OctocrabBuilder::default();
                if let Some(app) = &ctx.app {
                    builder.app(app.app_id, app.key.clone()).build()
                } else {
                    builder.build()
                }
                .map_err(Error::Octocrab)?
            }
            None => Octocrab::default(),
        };
        let instance = match  ctx.and_then(|ctx| ctx.app).map(|app| app.installation_id){
            Some(installation_id) =>  
                instance
                .installation(installation_id)
                .map_err(Error::Octocrab)?,
            None => instance,
        };
        Ok(Self{octocrab: instance})
       
    }

    pub async fn get_content(
        &self,
        repository: &Repository,
        path: &str,
    ) -> Result<ContentItems, Error> {
        info!("Try to get {path} content for {repository}");
        self.octocrab
            .repos(&repository.owner, &repository.name)
            .get_content()
            .path(path)
            .r#ref("main")
            .send()
            .await
            .map_err(Error::Octocrab)
    }
}
