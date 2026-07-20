use topcoat::{
    Result,
    view::{component, view},
};

use crate::todo::Todo;

#[component]
pub async fn intro() -> Result {
    view! {
        <p
            class="mb-4 rounded-md bg-blue-50 px-3 py-2 text-sm text-blue-900 ring-1 ring-blue-100"
        >
            "This is a demo of "
            <strong>"progressive enhancement"</strong>
            ": every form below is plain HTML that fully works with JavaScript disabled. "
            "Alpine AJAX layers on top to make the same forms feel instant -- partial page "
            "updates, inline validation, and a dismissible alert -- without changing how "
            "they're built. Try the "
            <strong>"Disable JavaScript"</strong>
            " button above to see the fallback."
        </p>
    }
}

// Lives outside the areas any form explicitly targets. Because it carries
// `x-sync`, Alpine AJAX refreshes it whenever a response includes a matching
// `id="alert"` element, whether or not that response's `x-target` named it.
// The message itself carries its own `x-data`, so it dismisses itself a few
// seconds after being merged in, regardless of which request produced it.
//
// The dismiss link is progressively enhanced: with JavaScript, `x-on:click.prevent`
// hides it immediately by flipping `show`. Note this uses the unabbreviated
// `x-on:` form rather than Alpine's `@` shorthand -- `view!` reserves a leading
// `@` on an attribute for Topcoat's own runtime event handlers, so a literal
// `@click` here would never reach Alpine at all. Without JavaScript, the
// attribute is just inert markup, so the click falls through to a plain
// navigation to `/`; since the flash message backing this alert is read (and
// cleared) exactly once, that reload lands on a page with nothing left to show.
#[component]
pub async fn alert(message: Option<&str>, error: bool) -> Result {
    view! {
        <div id="alert" x-sync=(true) role="status">
            match message {
                Some(message) => {
                    <div
                        x-data="{ show: true }"
                        x-init="setTimeout(() => show = false, 4000)"
                        x-show="show"
                        class=(alert_classes(error))
                    >
                        <p>(message)</p>
                        <a
                            href="/"
                            x-on:click.prevent="show = false"
                            class="shrink-0 text-xs font-medium underline hover:no-underline"
                        >
                            "Dismiss"
                        </a>
                    </div>
                }
                None => "",
            }
        </div>
    }
}

fn alert_classes(error: bool) -> &'static str {
    if error {
        "mb-4 flex items-center justify-between gap-3 rounded-md bg-red-50 px-3 py-2 text-sm text-red-700 ring-1 ring-red-200"
    } else {
        "mb-4 flex items-center justify-between gap-3 rounded-md bg-emerald-50 px-3 py-2 text-sm text-emerald-700 ring-1 ring-emerald-200"
    }
}

#[component]
pub async fn add_todo_form(title: &str, error: Option<&str>) -> Result {
    view! {
        <form
            id="add_todo_form"
            x-target="add_todo_form todo_list"
            ("x-target.422")="add_todo_form"
            method="post"
            action="/todos"
            class="mb-4 flex flex-col gap-1"
        >
            <label for="title" class="text-sm font-medium text-slate-700">"Task"</label>
            <div class="flex gap-2">
                <input
                    id="title"
                    name="title"
                    value=(title)
                    maxlength="60"
                    required=(true)
                    autocomplete="off"
                    autofocus=(true)
                    x-autofocus=(true)
                    class="flex-1 rounded-md border border-slate-300 px-2 py-1 text-sm focus:border-blue-500 focus:outline-none"
                    if error.is_some() {
                        aria-describedby="title_error"
                    }
                >
                <button
                    type="submit"
                    class="rounded-md bg-blue-600 px-3 py-1 text-sm font-medium text-white hover:bg-blue-500"
                >
                    "Add"
                </button>
            </div>
            match error {
                Some(error) => <p id="title_error" class="text-xs text-red-600">
                    (error)
                </p>,
                None => "",
            }
        </form>
    }
}

#[component]
pub async fn todo_list(todos: &[Todo]) -> Result {
    view! {
        if todos.is_empty() {
            <p
                id="todo_list"
                class="rounded-md border border-dashed border-slate-300 px-3 py-6 text-center text-sm text-slate-500"
            >
                "No todos yet -- add one above."
            </p>
        } else {
            <ul
                id="todo_list"
                class="divide-y divide-slate-200 rounded-md border border-slate-200 bg-white"
            >
                for todo in todos {
                    <li>todo_item(todo: todo)</li>
                }
            </ul>
        }
    }
}

// The checkbox and the title both sit inside one `<label>`, so clicking
// anywhere in the row (not just the checkbox itself) toggles it -- the
// standard, JavaScript-free way to make a whole row a checkbox's hit area.
#[component]
async fn todo_item(todo: &Todo) -> Result {
    view! {
        <form
            method="post"
            action=(("/todos/", todo.id, "/toggle"))
            x-target="todo_list"
        >
            <label
                class="flex cursor-pointer items-center gap-2 px-3 py-2 hover:bg-slate-50"
            >
                <input
                    type="checkbox"
                    checked=(todo.done)
                    onchange="this.form.requestSubmit()"
                    class="h-4 w-4 accent-blue-600"
                >
                if todo.done {
                    <s class="text-slate-400">(&todo.title)</s>
                } else {
                    <span>(&todo.title)</span>
                }
            </label>
        </form>
    }
}

#[component]
pub async fn clear_done_form() -> Result {
    view! {
        <form
            method="post"
            action="/todos/clear-done"
            x-target="todo_list"
            class="mt-3"
        >
            <button
                type="submit"
                class="text-xs font-medium text-slate-500 underline hover:text-slate-700"
            >
                "Clear Done"
            </button>
        </form>
    }
}
