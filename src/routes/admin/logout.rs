//! src/routes/admin/logout.rs

use crate::session_state::TypedSession;
use crate::utils::{error500, see_other};
use actix_web::{HttpResponse, post};
use actix_web_flash_messages::FlashMessage;

#[post("/admin/logout")]
pub async fn log_out(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(error500)?.is_none() {
        Ok(see_other("/login"))
    } else {
        session.log_out();
        FlashMessage::info("You have successfully logged out.").send();
        Ok(see_other("/login"))
    }
}
