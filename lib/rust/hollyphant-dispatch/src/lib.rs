mod request;
mod response;

use crate::request::{Request, RequestNewAccount, RequestNewAccountMastodon};
use crate::response::{Event, Response, Status};
use hollyphant::Hollyphant;
use log::warn;
use serde::Serialize;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::runtime::Runtime;

pub trait ErrorFormatter {
    fn format_error_mas_application_register(instance: &str) -> String;
}

pub trait EventPublisher {
    fn publish_set(&self, key: &[u8], value: &[u8]);
}

pub struct HollyphantDispatch<EF>
where
    EF: ErrorFormatter,
{
    runtime: Runtime,
    hollyphant: Arc<Hollyphant>,
    phantom: PhantomData<EF>,
}

impl<EF> HollyphantDispatch<EF>
where
    EF: ErrorFormatter,
{
    pub fn new(runtime: Runtime, hollyphant: Arc<Hollyphant>) -> Self {
        Self {
            runtime,
            hollyphant,
            phantom: PhantomData,
        }
    }

    pub fn execute<EP>(&self, publisher: EP, key: &[u8], args: &[u8])
    where
        EP: EventPublisher + Send + Sync + 'static,
    {
        let key = key.to_vec();
        if let Some(request) = Request::try_parse(&key, args) {
            publish_start(&publisher, &key);
            let hollyphant = self.hollyphant.clone();
            self.runtime.spawn(async move {
                let event = Self::dispatch(&hollyphant, request).await;
                publish_event(publisher, key, event);
            });
        }
    }

    async fn dispatch(hollyphant: &Hollyphant, request: Request) -> Event {
        match request {
            Request::Init => {
                let initial_state = hollyphant.initial_state();
                Event::Set(Status::success(Response::InitialState(initial_state)))
            }
            Request::NewAccount(new_account) => {
                Self::dispatch_new_account(hollyphant, new_account).await
            }
        }
    }

    async fn dispatch_new_account(hollyphant: &Hollyphant, request: RequestNewAccount) -> Event {
        match request {
            RequestNewAccount::Mastodon(request) => match request {
                RequestNewAccountMastodon::OpenBrowser { args } => {
                    let result = hollyphant.mas_pre_login(&args).await.map(Response::String);
                    Event::Set(Status::from_result::<EF>(result))
                }
            },
        }
    }
}

fn publish_start<EP>(publisher: &EP, key: &[u8])
where
    EP: EventPublisher,
{
    serialize_and_consume(Status::InProgress, |value| {
        publisher.publish_set(key, &value)
    });
}

fn publish_event<EP>(publisher: EP, key: Vec<u8>, event: Event)
where
    EP: EventPublisher,
{
    match event {
        Event::Set(status) => {
            serialize_and_consume(status, |value| publisher.publish_set(&key, &value))
        }
    }
}

fn serialize_and_consume<T, F>(value: T, f: F)
where
    T: Serialize,
    F: FnOnce(Vec<u8>),
{
    match serde_json::to_vec(&value) {
        Ok(value) => f(value),
        Err(error) => {
            warn!("Could not serialize response: {error}");
        }
    }
}
