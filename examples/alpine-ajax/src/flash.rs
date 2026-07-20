use serde::{Deserialize, Serialize};
use topcoat::{Result, context::Cx, cookie::cookie_store};

use crate::cookies;

// A message flashed across the Post/Redirect/Get fallback for clients
// without JavaScript, since a redirect response cannot carry a body of its
// own. Stored as a short-lived cookie and read exactly once. `title` is only
// meaningful alongside a validation `error`, to refill the input; every other
// case leaves it empty.
#[derive(Clone, Serialize, Deserialize)]
pub struct Flash {
    pub title: String,
    pub message: String,
    pub error: bool,
}

pub fn take_flash(cx: &Cx) -> Result<Option<Flash>> {
    let Some(store) = cookie_store::<Flash, _>(cookies(cx), "flash").parse()? else {
        return Ok(None);
    };
    let flash = store.get();
    store.remove();
    Ok(Some(flash))
}

pub fn set_flash(cx: &Cx, flash: Flash) -> Result<()> {
    cookie_store(cookies(cx), "flash").set(flash).commit()?;
    Ok(())
}
