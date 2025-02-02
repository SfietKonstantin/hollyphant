mod request;
mod response;

use crate::request::{MastodonNewAccount, NewAccount, Request};
use crate::response::{Event, Response, Status};
use hollyphant::Hollyphant;
use log::warn;
use serde::Serialize;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

pub trait ErrorFormatter {
    fn format_error_unexpected() -> String;
    fn format_error_mas_application_register(instance: &str) -> String;
    fn format_error_database() -> String;
}

pub trait EventPublisher {
    fn publish_set(&self, key: &[u8], value: &[u8]);
    fn publish_append(&self, key: &[u8], value: &[u8]);
}

pub struct HollyphantDispatch<EF>
where
    EF: ErrorFormatter,
{
    runtime: Runtime,
    hollyphant: Arc<Mutex<Hollyphant>>,
    phantom: PhantomData<EF>,
}

impl<EF> HollyphantDispatch<EF>
where
    EF: ErrorFormatter,
{
    pub fn new(runtime: Runtime, hollyphant: Arc<Mutex<Hollyphant>>) -> Self {
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
                let event = Self::dispatch(hollyphant, request).await;
                publish_events(publisher, key, event);
            });
        }
    }

    async fn dispatch(hollyphant: Arc<Mutex<Hollyphant>>, request: Request) -> Vec<Event> {
        match request {
            Request::Init => {
                let hollyphant = hollyphant.lock().await;
                let initial_state = hollyphant.initial_state();
                vec![Event::Set(Status::success(Response::InitialState(
                    initial_state,
                )))]
            }
            Request::NewAccount(new_account) => {
                Self::dispatch_new_account(hollyphant, new_account).await
            }
        }
    }

    async fn dispatch_new_account(
        hollyphant: Arc<Mutex<Hollyphant>>,
        request: NewAccount,
    ) -> Vec<Event> {
        let mut hollyphant = hollyphant.lock().await;
        match request {
            NewAccount::Mastodon(request) => match request {
                MastodonNewAccount::Prelogin { args } => {
                    let result = hollyphant
                        .mas_pre_login(args.name, args.instance)
                        .await
                        .map(Response::MasOAuthUrl);
                    vec![Event::Set(Status::from_result::<EF>(result))]
                }
                MastodonNewAccount::Login { args } => {
                    let result = hollyphant
                        .mas_login(args.name, args.code)
                        .await
                        .map(|_| Response::AccountCreated());
                    vec![Event::Set(Status::from_result::<EF>(result))]
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

fn publish_events<EP>(publisher: EP, key: Vec<u8>, events: Vec<Event>)
where
    EP: EventPublisher,
{
    for event in events {
        publish_event(&publisher, &key, event)
    }
}

fn publish_event<EP>(publisher: &EP, key: &[u8], event: Event)
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
