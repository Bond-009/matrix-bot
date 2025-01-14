use ruma::{OwnedRoomId, OwnedUserId};
use std::collections::HashSet;

#[derive(Debug)]
pub struct MatrixMessage {
    pub room_id: Option<OwnedRoomId>,
    pub message: MatrixMessageType,
    // pub resp: Responder<MatrixMessageResult>,
}

#[derive(Debug)]
pub enum MatrixMessageType {
    Invite(MatrixInviteMessage),
    Text(String),
    Notice(String),
    FormattedText(MatrixFormattedMessage),
    FormattedNotice(MatrixFormattedMessage),
    Ban(MatrixBanMessage),
}

#[derive(Debug)]
pub enum MatrixInviteType {
    Accept,
    Reject,
}

#[derive(Debug)]
pub struct MatrixFormattedMessage {
    pub plain_text: String,
    pub formatted_text: Option<String>,
}

#[derive(Debug)]
pub struct MatrixInviteMessage {
    pub kind: MatrixInviteType,
    pub sender: OwnedUserId,
}

#[derive(Debug)]
pub struct MatrixBanMessage {
    pub user: OwnedUserId,
    pub reason: Option<String>,
    pub rooms: HashSet<OwnedRoomId>,
}

// #[derive(Debug)]
// pub enum MatrixMessageResult {
//     Sent,
//     FailedToSend,
// }

// pub type Responder<T> = oneshot::Sender<T>;
