use anyhow::Context;
use std::io::StdoutLock;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub type Source = String;
pub type Destination = String;

pub type MessageId = usize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message<Payload> {
    pub src: Source,
    #[serde(rename = "dest")]
    pub dst: Destination,
    pub body: Body<Payload>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body<Payload> {
    #[serde(rename = "msg_id")]
    pub id: Option<MessageId>,
    pub in_reply_to: Option<MessageId>,
    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Init {
    node_id: String,
    node_ids: Vec<String>,
}

pub trait Node<Payload> {
    fn step(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> anyhow::Result<()>;
}

pub fn main_loop<S, Payload>(mut state: S) -> anyhow::Result<()>
where
    S: Node<Payload>,
    Payload: DeserializeOwned,
{
    let stdin = std::io::stdin().lock();
    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message<Payload>>();

    let mut stdout = std::io::stdout().lock();

    for input in inputs {
        let input = input?;

        state
            .step(input, &mut stdout)
            .context("Node step function failed")?;
    }

    Ok(())
}
