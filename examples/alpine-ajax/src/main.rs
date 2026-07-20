mod components;
mod flash;
mod js_toggle;
mod pages;
mod routes;
mod todo;

use topcoat::{
    asset::{AssetBundle, RouterBuilderAssetExt},
    context::Cx,
    cookie::{Cookies, RouterBuilderCookieExt},
    router::{Router, RouterBuilderDiscoverExt},
};

use crate::todo::TodoList;

#[tokio::main]
async fn main() {
    topcoat::start(
        Router::builder()
            .discover()
            .cookies()
            .assets(AssetBundle::load().unwrap())
            .app_context(TodoList::new())
            .build(),
    )
    .await
    .unwrap();
}

// The app's cookie jar, defaulting every cookie's `Path` to `/`.
//
// Without this, a cookie's default path is derived from whatever route wrote
// it (e.g. `/todos/clear-done` defaults to `Path=/todos`), which then makes
// it invisible to a `GET /` request. Routing every cookie through one jar
// with a forced default keeps that consistent regardless of which handler
// sets it.
pub fn cookies(cx: &Cx) -> impl Cookies {
    topcoat::cookie::cookies(cx).default_path("/")
}
