use yew::{Component, Context, Html, html};

pub enum Msg {
    Increment,
    Decrement,
}

pub struct App {
    value: isize
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.value += 1;
                true
            },
            Msg::Decrement => {
                self.value -= 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <main>
                <button onclick={ctx.link().callback(|_| Msg::Increment)}>
                    {"+"}
                </button>
                <button onclick={ctx.link().callback(|_| Msg::Decrement)}>
                    {"+"}
                </button>
                <p>{ self.value }</p>
            </main>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}