use log::info;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
struct TodoAppProp {
    current_route: Route,
}

#[function_component(TodoApp)]
fn todo_app(props: &TodoAppProp) -> Html {
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
                    let value = input.value().trim().to_string();

                    if value.len() > 0 {
                        let mut todos_arr = (*todos).clone();
                        let id: usize = rng.gen();
                        todos_arr.push(Todo::new(id, value));
                        todos.set(todos_arr);

                        input.set_value("");
                    }
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

            todos.set(todos_arr);
        })
    };

    let ontoggleall =
        {
            let todos = todos.clone();

            Callback::from(move |_| {
                let mut todos_arr: Vec<Todo> = (*todos).clone();

                let all_selected = todos_arr.iter().all(|todo| todo.completed);

                todos_arr.iter_mut().for_each(|t| {
                    t.completed = !all_selected.clone();
                });

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

    let num_items_left = (*todos).iter().filter(|todo| !todo.completed).count();

    let clear_completed = {
        let todos = todos.clone();
        Callback::from(move |_| {
            let mut todos_arr: Vec<Todo> = (*todos).clone();
            todos_arr.retain_mut(|t| !(*t).completed);

            todos.set(todos_arr);
        })
    };

    html! {
        <section class="todoapp">
            <TodoHeader onkeydown={onnew} />
            <TodoMain
              current_route={props.current_route.clone()}
              todos={(*todos).clone()}
              ontoggle={ontoggle}
              onremove={onremove}
              onupdate={onupdate}
              ontoggleall={ontoggleall}  />
            <TodoFooter current_route={props.current_route.clone()} num_items_left={num_items_left} clear_completed={clear_completed}/>
        </section>
    }
}

#[function_component(TodoMain)]
fn todo_main(props: &TodosProps) -> Html {
    let active_todo =
        props
            .todos
            .iter()
            .filter(|todo| match props.current_route {
                Route::All => true,
                Route::Active => !todo.completed,
                Route::Completed => todo.completed,
            })
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
            <input id="toggle-all" class="toggle-all" type="checkbox" onclick={props.ontoggleall.clone()} />
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
                if value.len() == 0 {
                    todo.title = value;
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
                <input class="toggle" type="checkbox" checked={props.todo.completed} onchange={move |e: Event| {
                    props.ontoggle.emit(todo.clone());
                    e.prevent_default();
                }} />
                <label ondblclick={handle_double_click}>{&todo_title}</label>
                <button class="destroy" onclick={move |_| props.onremove.emit(todo_id)} />
            </div>

            if *editing {
            <div class="input-container">
                <input class="edit" id="edit-todo-input" ref={input_ref} onblur={handle_submit} onkeydown={handle_keydown} value={todo_title} />
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
            <input class={"new-todo"} placeholder={"What needs to be done?"} onkeydown={props.onkeydown.clone()}/>
        </header>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct TodoFooterProps {
    clear_completed: Callback<MouseEvent>,
    current_route: Route,
    num_items_left: usize,
}

#[function_component(TodoFooter)]
fn todo_footer(props: &TodoFooterProps) -> Html {
    html! {
        <footer class="footer">
            <span class="todo-count"><strong>{props.num_items_left}</strong>{" item left!"}</span>
                <ul class="filters">
                    <li>
                        <a href="active" class={(props.current_route == Route::All).then(|| Some("selected"))}>{"All"}</a>
                    </li>
                    <li>
                        <a href="active" class={(props.current_route == Route::Active).then(|| Some("selected"))}>{"Active"}</a>
                    </li>
                    <li>
                        <a href="completed" class={(props.current_route == Route::Completed).then(|| Some("selected"))}>{"Completed"}</a>
                    </li>
                </ul>
            <button class="clear-completed" onclick={props.clear_completed.clone()}>{"Clear completed"}</button>
        </footer>
    }
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    All,
    #[at("/active")]
    Active,
    #[at("/completed")]
    Completed,
}

fn switch(route: Route) -> Html {
    html! { <TodoApp current_route={route} /> }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}

#[derive(Properties, PartialEq, Clone)]
struct TodosProps {
    ontoggleall: Callback<MouseEvent>,
    current_route: Route,
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
