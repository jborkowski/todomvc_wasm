use log::info;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let todos = use_state(|| vec![]);

    let on_new_todo = {
        let todos = todos.clone();

        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "Enter" {
                let input = event.target_dyn_into::<HtmlInputElement>();

                if let Some(input) = input {
                    let todos = todos.clone();

                    let mut prev = (*todos).clone();
                    prev.push(Todo::new(input.value()));

                    todos.set(prev);
                    info!("{}", input.value());
                }
            }
        })
    };

    html! {
        <div>
            <TodoHeader onkeydown={on_new_todo}  />
            <TodoMain todos={(*todos).clone()}/>
        </div>
    }
}

#[function_component(TodoMain)]
fn todo_main(props: &TodosProps) -> Html {
    let active_todo = props
        .todos
        .iter()
        .filter(|todo| !todo.completed)
        .enumerate()
        .map(|(index, todo)| {
            html! {
                <TodoItem todo={(*todo).clone()} index={index}/>
            }
        })
        .collect::<Html>();

    html! {
        <div>
          <div>
            <input type="checkbox" />
          </div>
          <ul>
            {active_todo}
          </ul>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct TodoItemProps {
    // on_save: Callback<Todo>,
    // on_remove: Callback<usize>,
    // on_toggle: Callback<usize>,
    todo: Todo,
    index: usize,
}

#[function_component(TodoItem)]
fn todo_item(props: &TodoItemProps) -> Html {
    html! {
        <li>
        <div>
        {props.todo.title.clone()}
        </div>
        </li>
    }
}

#[derive(Properties, PartialEq)]
pub struct TodoHeaderProps {
    pub onkeydown: Callback<KeyboardEvent>,
}

#[function_component(TodoHeader)]
fn todo_header(props: &TodoHeaderProps) -> Html {
    html! {
        <header class="header">
            <h1>{"todos"}</h1>
            <input class={"new-todo"} placeholder={"What needs to be done?"} onkeydown={props.onkeydown.clone()} />
        </header>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}

#[derive(Properties, PartialEq)]
struct TodosProps {
    todos: Vec<Todo>,
}

#[derive(Properties, PartialEq, Clone)]
struct Todo {
    title: String,
    completed: bool,
}

impl Todo {
    pub fn new(title: String) -> Todo {
        Todo {
            title,
            completed: false,
        }
    }
}
