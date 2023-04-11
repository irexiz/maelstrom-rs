use serde::{Deserialize, Serialize};

pub type Source = String;
pub type Destination = String;

pub type MessageId = usize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub src: Source,
    #[serde(rename = "dest")]
    pub dst: Destination,
    pub body: Body,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    #[serde(rename = "msg_id")]
    pub id: Option<MessageId>,
    pub in_reply_to: Option<MessageId>,
    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
}
