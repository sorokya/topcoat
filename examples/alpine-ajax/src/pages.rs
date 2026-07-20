use topcoat::{
    Result,
    alpine_ajax::ajax_request,
    context::Cx,
    router::{Slot, layout, page},
    tailwind,
    view::view,
};

use crate::{
    components::{add_todo_form, alert, clear_done_form, intro, todo_list},
    flash::take_flash,
    js_toggle::{js_toggle_form, scripts_disabled},
    todo::todos,
};

#[layout("/")]
async fn root(cx: &Cx, slot: Slot<'_>) -> Result {
    // Alpine AJAX only merges the requested target elements, so for
    // client-side navigations we don't need to return the full HTML shell
    // again.
    if ajax_request(cx) {
        return slot.await;
    }

    view! {
        <!DOCTYPE html>
        <html>
            <head>
                <title>"Topcoat+Alpine Todos"</title>
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
                if !scripts_disabled(cx) {
                    <script
                        defer=(true)
                        src="https://cdn.jsdelivr.net/npm/@imacrayon/alpine-ajax@0.12.4/dist/cdn.min.js"
                    >

                    </script>
                    <script
                        defer=(true)
                        src="https://cdn.jsdelivr.net/npm/alpinejs@3.15.0/dist/cdn.min.js"
                    >

                    </script>
                }
                topcoat::dev::script()
            </head>
            <body class="min-h-screen bg-slate-100 font-sans text-slate-900">
                <main class="mx-auto max-w-md p-6">(slot.await?)</main>
            </body>
        </html>
    }
}

#[page("/")]
async fn home(cx: &Cx) -> Result {
    let flash = take_flash(cx)?;
    let title = flash.as_ref().map_or("", |flash| flash.title.as_str());
    let message = flash.as_ref().map(|flash| flash.message.as_str());
    let error = flash.as_ref().is_some_and(|flash| flash.error);
    let todos = todos(cx).snapshot();

    view! {
        <div class="mb-4 flex items-center justify-between">
            <h1 class="text-2xl font-bold tracking-tight">"Topcoat+Alpine Todos"</h1>
            js_toggle_form(enabled: !scripts_disabled(cx))
        </div>

        intro()

        alert(message: message, error: error)

        add_todo_form(title: title, error: if error { message } else { None })

        todo_list(todos: &todos)

        clear_done_form()
    }
}
