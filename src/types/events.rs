use super::content::{Content};

#[derive(Serialize, Deserialize, Debug)]
pub struct UnsignedData {
    pub age: u64,
    pub prev_content: Option<Content>,
    pub prev_sender: Option<String>,
    pub txn_id: Option<String>,
    pub redacted_because: Option<::serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
/// Events like m.typing
pub struct EphemeralEvent {
    #[serde(rename="type")]
    pub event_type: String,
    pub content: Content,
    pub room_id: Option<String>,
    pub event_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
/// Event in invite_room_state
pub struct InviteStateEvent {
    #[serde(rename="type")]
    pub event_type: String,
    pub content: Content,
    pub sender: Option<String>,
    pub state_key: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
/// A redact event
pub struct RedactedEvent {
    // event
    #[serde(rename="type")]
    pub event_type: String,
    pub content: Content,
    pub prev_sender: Option<String>,
    pub prev_content: Option<Content>,
    pub event_id: Option<String>,
    pub room_id: Option<String>,
    pub sender: Option<String>,
    pub redacted_because: Event
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
/// Ephemeral events (like m.typing). of course, that could be included in Event, but then we have three more values being wrapped in Option.
pub struct MinimalEvent {
    #[serde(rename="type")]
    pub event_type: String,
    pub content: Content,
    pub room_id: Option<String>,
    pub event_id: Option<String>,
    pub sender: Option<String>,
    pub state_key: Option<String>,
}
/// An event in a room.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
// #[cfg_attr(features="must_match_fields",
pub struct Event {
    // event
    #[serde(rename="type")]
    pub event_type: String,
    pub content: Content,
    // room event
    pub event_id: String,
    pub sender: String,
    pub origin_server_ts: u64,
    pub room_id: Option<String>,
    // can be recursive until we differ between redacted and not redacted events
    pub unsigned: Option<UnsignedData>,
    // state event
    pub state_key: Option<String>,
    pub prev_content: Option<Content>,
    pub prev_sender: Option<String>,
    pub invite_room_state: Option<Vec<InviteStateEvent>>,
    // extra
    pub age: Option<u64>,
    pub txn_id: Option<String>,
    pub redacts: Option<String>,
    pub membership: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
// #[serde(deny_unknown_fields)]
/// for now events have to exactly follow the specs
/// the different event types
pub enum EventTypes {
    Event(Event),
    RedactedEvent(RedactedEvent),
    MinimalEvent(MinimalEvent),
}


/// Events in a room.
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Events {
    pub events: Vec<EventTypes>
}