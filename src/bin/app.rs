fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<todomvc_wasm::App>::new().render();
}
