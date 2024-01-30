use log::info;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    use rand::Rng;
    let rng = rand::thread_rng();
    let todos = use_state(|| vec![]);

    let onnew = {
        let todos = todos.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "Enter" {
                let input = event.target_dyn_into::<HtmlInputElement>();
                let mut rng = rng.clone();

                if let Some(input) = input {
                    let mut todos_arr = (*todos).clone();
                    let id: usize = rng.gen();
                    todos_arr.push(Todo::new(id, input.value()));

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

    let onremove =
        {
            let todos = todos.clone();
            Callback::from(move |todo_id| {
                let mut todos_arr: Vec<Todo> = (*todos).clone();
                todos_arr.retain_mut(|t| (*t).id != todo_id);

                todos.set(todos_arr);
            })
        };

    let onupdate = {
        let todos = todos.clone();

        Callback::from(move |todo: Todo| {
            let mut todos_arr: Vec<Todo> = (*todos).clone();

            todos_arr.iter_mut().for_each(|t| {
                if (*t).id == todo.id {
                    t.update(&todo);
                }
            });

            todos.set(todos_arr);
        })
    };

    html! {
        <section class="todoapp">
            <TodoHeader onkeydown={onnew} />
            <TodoMain
              todos={(*todos).clone()}
              ontoggle={ontoggle}
              onremove={onremove}
              onupdate={onupdate} />
            <TodoFooter />
        </section>
    }
}

#[function_component(TodoMain)]
fn todo_main(props: &TodosProps) -> Html {
    let active_todo = props
        .todos
        .iter()
        // .filter(|todo| !todo.completed)
        .map(|todo| {
            let props = props.clone();
            html! {
                <TodoItem
                  todo={todo.clone()}
                  onupdate={move |todo| props.onupdate.emit(todo)}
                  ontoggle={move |todo| props.ontoggle.emit(todo)}
                  onremove={move |todo_id| props.onremove.emit(todo_id)} />

            }
        })
        .collect::<Html>();

    html! {
        <section class="main">
            <input id="toggle-all" class="toggle-all" type="checkbox" />
            <label for="toggle-all">{"Mark all as complete"}</label>
            <ul class="todo-list"> {active_todo} </ul>
        </section>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct TodoItemProps {
    onupdate: Callback<Todo>,
    onremove: Callback<usize>,
    ontoggle: Callback<Todo>,
    todo: Todo,
}

#[function_component(TodoItem)]
fn todo_item(props: &TodoItemProps) -> Html {
    let editing = use_state(|| false);
    let input_ref = use_node_ref();

    // {
    //     let input_ref = input_ref.clone();
    //     use_effect_with(input_ref, |input_ref| {
    //         let input = input_ref
    //             .cast::<HtmlInputElement>()
    //             .expect("input_ref not attached to input element");

    //         input.focus();

    //         let listener = Closure::<dyn Fn(Event)>::wrap(Box::new(|event| {
    //             web_sys::console::log_1(&"Clicked!".into());
    //         }));

    //         input
    //             .add_event_listener_with_callback("click", listener.as_ref().unchecked_ref())
    //             .unwrap();
    //     })
    // }

    let handle_double_click =
        {
            let editing = editing.clone();
            move |_| {
                editing.clone().set(true);
            }
        };

    let handle_keydown = {
        let editing = editing.clone();
        let props = props.clone();

        move |event: KeyboardEvent| {
            let props = props.clone();

            info!("{}", event.key());
            if event.key() == "Escape" {
                editing.clone().set(false);
            } else if event.key() == "Enter" {
                let input = event.target_dyn_into::<HtmlInputElement>();

                if let Some(input) = input {
                    let mut todo = props.todo.clone();

                    let value = input.value().trim().to_string();
                    if value.len() > 0 {
                        todo.title = input.value();
                        props.onupdate.emit(todo);
                        editing.set(false);
                    } else {
                        props.onremove.emit(todo.id);
                    }
                }
            }
        }
    };

    let handle_submit = {
        let props = props.clone();
        let editing = editing.clone();
        move |event: FocusEvent| {
            let editing = editing.clone();
            let props = props.clone();

            let input = event.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                let mut todo = props.todo.clone();

                let value = input.value().trim().to_string();
                if value.len() > 0 {
                    todo.title = input.value();
                    props.onupdate.emit(todo);
                    editing.set(false);
                } else {
                    props.onremove.emit(todo.id);
                }
            }
        }
    };

    let props = props.clone();
    let todo_id = props.todo.id.clone();
    let todo_title = props.todo.title.clone();
    let todo = props.todo.clone();

    html! {
        <li class={classes!(props.todo.completed.then(|| Some("completed")), (*editing).then(|| Some("editing")))}>
            <div class="view">
                <input class="toggle" type="checkbox" checked={props.todo.completed} onchange={move |_| props.ontoggle.emit(todo.clone())} />
                <label ondblclick={handle_double_click}>{todo_title}</label>
                <button class="destroy" onclick={move |_| props.onremove.emit(todo_id)} />
            </div>

            if *editing {
            <div class="input-container">
                <input class="edit" id="edit-todo-input" ref={input_ref} onblur={handle_submit} onkeydown={handle_keydown} value={props.todo.title.clone()} />
                <label class="visually-hidden" htmlFor="edit-todo-input">
                    {"Edit Todo Input"}
                </label>
            </div>
            }
        </li>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct TodoHeaderProps {
    onkeydown: Callback<KeyboardEvent>,
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

#[derive(Properties, PartialEq, Clone)]
pub struct TodoFooterProps {}

#[function_component(TodoFooter)]
fn todo_footer(props: &TodoFooterProps) -> Html {
    html! {
        <footer class="footer">
            <span class="todo-count"><strong>{0}</strong>{"item left"}</span>
                <ul class="filters">
                    <li>
                        <a class="selected" href="#/">{"All"}</a>
                    </li>
                    <li>
                        <a href="#/active">{"Active"}</a>
                    </li>
                    <li>
                        <a href="#/completed">{"Completed"}</a>
                    </li>
                </ul>
            <button class="clear-completed">{"Clear completed"}</button>
        </footer>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}

#[derive(Properties, PartialEq, Clone)]
struct TodosProps {
    todos: Vec<Todo>,
    onupdate: Callback<Todo>,
    ontoggle: Callback<Todo>,
    onremove: Callback<usize>,
}

#[derive(Properties, PartialEq, Clone, Debug)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
}

impl Todo {
    pub fn new(id: usize, title: String) -> Todo {
        Todo {
            id,
            title,
            completed: false,
        }
    }

    pub fn toggle(&mut self) {
        self.completed = !self.completed;
    }

    pub fn update(&mut self, todo: &Todo) {
        self.completed = todo.completed;
        self.title = todo.title.clone();
    }
}
