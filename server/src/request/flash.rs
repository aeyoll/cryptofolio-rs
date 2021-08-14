use actix_session::UserSession;
use actix_web::Error;
use actix_web::HttpRequest;

use serde::{Deserialize, Serialize};

/// A `FlashMessage` is a generic message that can be shoved into the Session
/// between requests. This isn't particularly useful for JSON-based workflows, but
/// for the traditional webapp side it works well.
#[derive(Debug, Deserialize, Serialize)]
pub struct FlashMessage {
    pub title: String,
    pub message: String,
}
pub trait FlashMessages {
    /// Adds a flash message to the stack.
    fn flash(&self, title: &str, message: &str) -> Result<(), Error>;

    /// Internally used; loads flash messages for template use and removes the existing
    /// stack.
    fn get_flash_messages(&self) -> Result<Vec<FlashMessage>, Error>;
}

impl FlashMessages for HttpRequest {
    fn flash(&self, title: &str, message: &str) -> Result<(), Error> {
        let session = self.get_session();

        let mut messages: Vec<FlashMessage> = match session.get("flsh")? {
            Some(messages) => messages,
            None => Vec::new(),
        };

        messages.push(FlashMessage {
            title: title.to_string(),
            message: message.to_string(),
        });
        session.set("flsh", messages)?;

        Ok(())
    }

    fn get_flash_messages(&self) -> Result<Vec<FlashMessage>, Error> {
        let session = self.get_session();

        let messages = match session.get("flsh")? {
            Some(messages) => messages,
            None => Vec::new(),
        };

        session.remove("flsh");
        Ok(messages)
    }
}
