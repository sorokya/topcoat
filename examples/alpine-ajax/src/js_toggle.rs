use topcoat::{
    Result,
    context::Cx,
    cookie::{Cookie, Cookies},
    router::{SeeOther, route, see_other},
    view::{component, view},
};

use crate::cookies;

// A cookie that lets a visitor disable JavaScript for this demo without
// digging through browser settings, so the same handlers can be exercised
// both with and without Alpine AJAX in the loop.

pub fn scripts_disabled(cx: &Cx) -> bool {
    cookies(cx).get("js").is_some()
}

#[route(POST "/js")]
async fn toggle_js(cx: &Cx) -> Result<SeeOther> {
    let jar = cookies(cx);
    match jar.get("js") {
        Some(_) => jar.remove(Cookie::build(("js", "")).path("/").build()),
        None => jar.add(Cookie::build(("js", "disabled")).path("/").build()),
    }

    Ok(see_other("/"))
}

#[component]
pub async fn js_toggle_form(enabled: bool) -> Result {
    view! {
        <form method="post" action="/js">
            <button
                type="submit"
                class="rounded-md bg-slate-200 px-2 py-1 text-xs font-medium text-slate-700 hover:bg-slate-300"
            >
                if enabled {
                    "Disable JavaScript"
                } else {
                    "Enable JavaScript"
                }
            </button>
        </form>
    }
}
