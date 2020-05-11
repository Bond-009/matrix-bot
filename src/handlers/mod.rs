mod correct_spelling;
mod helpers;
// mod roll;
mod unit_conversion;

use self::correct_spelling::correct_spelling_check;
use self::helpers::do_nothing;
#[allow(unused_imports)]
// use self::roll::roll;
use self::unit_conversion::unit_conversion;
use crate::session::SavedSession;
use crate::regex::{NO_BANG, UNIT_CONVERSION};

use anyhow::Result;
use log::debug;
use ruma_client::{
    events::room::message::TextMessageEventContent,
    identifiers::{RoomId, UserId},
    HttpsClient,
};

pub async fn handle_text_message(
    text: &TextMessageEventContent,
    sender: &UserId,
    room_id: &RoomId,
    client: &HttpsClient,
    session: &mut SavedSession,
) -> Result<()> {
    if NO_BANG.is_match(&text.body) {
        debug!("Entering spell check path...");
        correct_spelling_check(text, sender, room_id, client, session).await
    } else if UNIT_CONVERSION.is_match(&text.body.to_lowercase()) {
        debug!("Entering unit conversion path...");
        unit_conversion(text, room_id, client, session).await
    // } else if ROLL.is_match(&text.body.to_lowercase()) {
    //     debug!("Entering roll path...");
    //     roll(text, room_id, client, session).await
    } else {
        debug!("Entering do nothing path...");
        do_nothing().await
    }
}
