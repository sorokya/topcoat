use serde::Deserialize;
use topcoat::{
    Result,
    alpine_ajax::ajax_request,
    context::Cx,
    router::{Form, IntoResponse, Response, StatusCode, path_param, route, see_other},
    view::view,
};

use crate::{
    components::{add_todo_form, alert, todo_list},
    flash::{Flash, set_flash},
    todo::todos,
};

#[derive(Deserialize)]
struct NewTodo {
    title: String,
}

// Validates and creates a todo.
//
// A validation failure responds `422 Unprocessable Entity` with just the
// `#add_todo_form` fragment (input preserved, error inline) and an `#alert`
// summary. The form's `x-target.422="add_todo_form"` tells Alpine AJAX to
// merge that fragment on a 422 instead of ignoring it, without touching
// `#todo_list`.
//
// A successful, Alpine AJAX-issued submission responds `200 OK` and updates
// both `#add_todo_form` (cleared) and `#todo_list`, plus an `#alert`
// confirmation synced in via `x-sync`.
//
// Without JavaScript, neither status-code targeting nor `x-sync` apply, so
// both branches fall back to the Post/Redirect/Get pattern instead. The
// validation message can't ride along on a redirect, so it goes through a
// one-time `flash` cookie that the next `GET /` reads back.
#[route(POST "/todos")]
async fn add_todo(cx: &Cx, Form(new_todo): Form<NewTodo>) -> Result<Response> {
    let title = new_todo.title.trim();

    let error = if title.is_empty() {
        Some("Enter a task.")
    } else if title.chars().count() > 60 {
        Some("Keep it under 60 characters.")
    } else {
        None
    };

    if let Some(message) = error {
        if ajax_request(cx) {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                view! {
                    add_todo_form(title: title, error: Some(message))
                    alert(message: Some(message), error: true)
                }?,
            )
                .into_response(cx);
        }

        set_flash(
            cx,
            Flash {
                title: title.to_owned(),
                message: message.to_owned(),
                error: true,
            },
        )?;
        return see_other("/").into_response(cx);
    }

    let list = todos(cx).push(title);
    let confirmation = format!("Added \"{title}\" to your list.");

    if ajax_request(cx) {
        return view! {
            add_todo_form(title: "", error: None)
            todo_list(todos: &list)
            alert(message: Some(&confirmation), error: false)
        }?
        .into_response(cx);
    }

    set_flash(
        cx,
        Flash {
            title: String::new(),
            message: confirmation,
            error: false,
        },
    )?;
    see_other("/").into_response(cx)
}

#[path_param(error = bad_request)]
struct TodoId(u64);

#[route(POST "/todos/{todo_id}/toggle")]
async fn toggle_todo(cx: &Cx) -> Result<Response> {
    let list = todos(cx).toggle(*path_param::<TodoId>(cx)?);

    if ajax_request(cx) {
        return view! { todo_list(todos: &list) }?.into_response(cx);
    }

    see_other("/").into_response(cx)
}

#[route(POST "/todos/clear-done")]
async fn clear_done(cx: &Cx) -> Result<Response> {
    let (list, cleared) = todos(cx).clear_done();
    let message = format!(
        "Cleared {cleared} completed task{}.",
        if cleared == 1 { "" } else { "s" }
    );

    if ajax_request(cx) {
        return view! {
            todo_list(todos: &list)
            alert(message: Some(&message), error: false)
        }?
        .into_response(cx);
    }

    set_flash(
        cx,
        Flash {
            title: String::new(),
            message,
            error: false,
        },
    )?;
    see_other("/").into_response(cx)
}
