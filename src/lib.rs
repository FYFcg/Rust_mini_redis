#![feature(impl_trait_in_assoc_type)]

use anyhow::Ok;
use std::{collections::HashMap, sync::Mutex};
use volo::FastStr;
use volo_gen::volo::example::{GetItemResponse, RedisCommand};

pub struct S {
    pub map: Mutex<HashMap<String, String>>,
}

#[volo::async_trait]
impl volo_gen::volo::example::ItemService for S {
    async fn get_item(
        &self,
        _req: volo_gen::volo::example::GetItemRequest,
    ) -> ::core::result::Result<volo_gen::volo::example::GetItemResponse, ::volo_thrift::AnyhowError>
    {
        match _req.command {
            RedisCommand::Set => {
                self.map.lock().unwrap().insert(
                    _req.key.unwrap().into_string(),
                    _req.value.unwrap().into_string(),
                );
                Ok(GetItemResponse {
                    flag: true,
                    res: Some("OK".into()),
                })
            }
            RedisCommand::Get => {
                match self
                    .map
                    .lock()
                    .unwrap()
                    .get(&_req.key.unwrap().into_string())
                {
                    Some(v) => Ok(GetItemResponse {
                        flag: true,
                        res: Some(FastStr::from(v.clone())),
                    }),
                    None => Ok(GetItemResponse {
                        flag: false,
                        res: Some("None".into()),
                    }),
                }
            }
            RedisCommand::Del => {
                match self
                    .map
                    .lock()
                    .unwrap()
                    .remove(&_req.key.unwrap().into_string())
                {
                    Some(_) => Ok(GetItemResponse {
                        flag: false,
                        res: Some("None".into()),
                    }),

                    None => Ok(GetItemResponse {
                        flag: true,
                        res: Some("OK".into()),
                    }),
                }
            }
            RedisCommand::Ping => Ok(GetItemResponse {
                flag: true,
                res: Some("PONG".into()),
            }),
            RedisCommand::Publish => Ok(Default::default()),
            RedisCommand::Subscribe => Ok(Default::default()),
        }
    }
}

pub struct LogLayer;

impl<S> volo::Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(self, inner: S) -> Self::Service {
        LogService(inner)
    }
}

#[derive(Clone)]
pub struct LogService<S>(S);

#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for LogService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug,
    Cx: Send + 'static,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        let now = std::time::Instant::now();
        tracing::debug!("Received request {:?}", &req);
        let resp = self.0.call(cx, req).await;
        tracing::debug!("Sent response {:?}", &resp);
        tracing::info!("Request took {}ms", now.elapsed().as_millis());
        resp
    }
}

#[derive(Clone)]
pub struct FliterService<S>(S);
#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for FliterService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug,
    anyhow::Error: Into<S::Error>,
    Cx: Send + 'static,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        let info = format!("{:?}", req);
        if info.contains("fff") {
            return Err(anyhow::anyhow!("fff is not allowed").into());
        }
        self.0.call(cx, req).await
    }
}
pub struct FilterLayer;

impl<S> volo::Layer<S> for FilterLayer {
    type Service = FliterService<S>;

    fn layer(self, inner: S) -> Self::Service {
        FliterService(inner)
    }
}
