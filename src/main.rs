use std::{env, path::Path, sync::Arc};
use board::Board;
use solver::start_solver;
use warp::Filter;
use tokio::sync::{RwLock, Notify};
use serde_json::{json, Value};

mod board;
mod solver;
mod gui;

#[derive(Default)]
struct AppState {
    is_working: bool,
    result: Option<Value>,
    notify: Arc<Notify>,
}

type SharedState = Arc<RwLock<AppState>>;

static DOMAIN: [u8; 4] = [127, 0, 0, 1];
static PORT: u16 = 3000;

#[tokio::main]
async fn main() {
    let is_gui_disabled = env::var("RUST_NO_GUI")
        .map(|x| x == "1")
        .unwrap_or(false);

    println!("GUI is {}", if is_gui_disabled { "disabled" } else { "enabled" });

    let url = format!("http://{}:{}/", DOMAIN.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("."), PORT);

    let state: SharedState = Arc::new(RwLock::new(AppState::default()));

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "OPTIONS"])
        .allow_headers(vec!["content-type"]);

    let gui_dir = match is_gui_disabled {
        true => Path::new("").into(),
        false => gui::ensure_gui_ready().expect("Failed to prepare GUI"),
    };

    let public = warp::get()
        .and(warp::fs::dir(gui_dir));

    let is_working_route = warp::path("is_working")
        .and(warp::get())
        .and(with_state(state.clone()))
        .and_then(handle_is_working);

    let start_route = warp::path("start")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(state.clone()))
        .and_then(handle_start)
        .with(cors.clone());

    let solution_route = warp::path("solution")
        .and(warp::get())
        .and(with_state(state.clone()))
        .and_then(handle_solution);

    let options = warp::options()
        .map(warp::reply);

    let routes = is_working_route
        .or(options)
        .or(start_route)
        .or(solution_route)
        .or(public)
        .with(cors);

    println!("Server started at {}", url);

    warp::serve(routes).run((DOMAIN, PORT)).await;
}

fn with_state(
    state: SharedState,
) -> impl Filter<Extract = (SharedState,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

async fn handle_is_working(state: SharedState) -> Result<impl warp::Reply, warp::Rejection> {
    let state = state.read().await;

    let mut response = json!({
        "is_working": state.is_working,
    });

    if let Some(result) = &state.result {
        response.as_object_mut().unwrap().insert("current_solution".to_string(), json!(result));
    }

    Ok(warp::reply::json(&response))
}

async fn handle_start(body: Board, state: SharedState) -> Result<impl warp::Reply, warp::Rejection> {
    let should_run_task = {
        let mut state = state.write().await;
        if state.is_working {
            return Ok(warp::reply::with_status("Server is already working", warp::http::StatusCode::CONFLICT));
        }
        state.is_working = true;
        state.notify.notify_waiters();
        true
    };

    if should_run_task {
        tokio::spawn(run_solution_finder(state.clone(), body));
    }

    Ok(warp::reply::with_status("Task started", warp::http::StatusCode::OK))
}

async fn run_solution_finder(state: SharedState, board: Board) {
    println!("Starting the task...");

    let solution = start_solver(board);
    let solution_json = serde_json::to_value(&solution).unwrap();

    println!("Task finished!");

    let mut state = state.write().await;
    state.is_working = false;
    state.result = Some(solution_json);
    state.notify.notify_waiters();
}

async fn handle_solution(state: SharedState) -> Result<impl warp::Reply, warp::Rejection> {
    let (is_working, notify) = {
        let state = state.read().await;
        (state.is_working, state.notify.clone())
    };

    if is_working {
        notify.notified().await;
    }

    let state = state.read().await;
    let result = state.result.clone().unwrap();

    Ok(warp::reply::json(&json!({ "solution": result })))
}