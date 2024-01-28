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
                    let mut todos_arr = (*todos).clone();
                    todos_arr.push(Todo::new(input.value()));

                    todos.set(todos_arr);
                }
            }
        })
    };

    let ontoggle = {
        let todos = todos.clone();

        Callback::from(move |todo| {
            let mut todos_arr: Vec<Todo> = (*todos).clone();

            todos_arr.iter_mut().for_each(|t| {
                if *t == todo {
                    t.toggle();
                }
            });

            info!("Toggled {:?}", todos_arr.clone());
            todos.set(todos_arr);
        })
    };

    html! {
        <div>
            <TodoHeader onkeydown={on_new_todo}  />
            <TodoMain todos={(*todos).clone()} ontoggle={ontoggle}/>
        </div>
    }
}

#[function_component(TodoMain)]
fn todo_main(props: &TodosProps) -> Html {
    let ontoggle = {
        let ontoggle = props.ontoggle.clone();
        move |todo| {
            let ontoggle = ontoggle.clone();
            ontoggle.emit(todo)
        }
    };

    let active_todo =
        props
            .todos
            .iter()
            .filter(|todo| !todo.completed)
            .map(|todo| {
                html! {
                    <TodoItem todo={(*todo).clone()}  ontoggle={ontoggle.clone()} />
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

#[derive(Properties, PartialEq, Clone)]
struct TodoItemProps {
    // on_save: Callback<Todo>,
    // on_remove: Callback<usize>,
    ontoggle: Callback<Todo>,
    todo: Todo,
}

#[function_component(TodoItem)]
fn todo_item(props: &TodoItemProps) -> Html {
    let handle_toggle = {
        let props = props.clone();
        move |_| {
            let props = props.clone();
            props.ontoggle.emit(props.todo.clone())
        }
    };

    let handle_double_click = |event: MouseEvent| info!("On Edit click {:?}", event);
    let handle_remove = |event| info!("On Remove click {:?}", event);

    html! {
        <li>
            <div class="view">
                <input class="toggle" type="checkbox" checked={props.todo.completed} onchange={handle_toggle} />
                <label ondblclick={handle_double_click}>{props.todo.title.clone()}</label>
                <button class="destroy" onclick={handle_remove} />
            </div>
        </li>
    }
}

#[derive(Properties, PartialEq, Clone)]
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

#[derive(Properties, PartialEq, Clone)]
struct TodosProps {
    todos: Vec<Todo>,
    ontoggle: Callback<Todo>,
}

#[derive(Properties, PartialEq, Clone, Debug)]
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

    pub fn toggle(&mut self) {
        self.completed = !self.completed;
    }
}
