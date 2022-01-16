use web_sys::HtmlTextAreaElement;
use yew::{html, Component, Context, Html, InputEvent, TargetCast};

enum Msg {
    Input(InputEvent),
}

struct Model {
    input: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input: String::from("日本語の文章をいい感じに分割します。"),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Input(e) => {
                self.input = e.target_unchecked_into::<HtmlTextAreaElement>().value();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let model = budoux::models::default_japanese_model();
        let words = budoux::parse(model, &self.input);

        html! {
            <div>
                <header class="container">
                <h1>{ "BudouX-rs Demo" }</h1>
                <p>
                    { "BudouX-rs is a rust port of " }
                    <a href="https://github.com/google/budoux">{ "BudouX" }</a>
                    { " (machine learning powered line break organizer tool)." }
                </p>
                <p>
                    { "[" }<a href="https://github.com/sg0hsmt/budoux-rs">{ "GitHub" }</a>{ "]" }
                    { "[" }<a href="https://crates.io/crates/budoux">{ "Crates.io" }</a>{ "]" }
                </p>
                </header>
                <main class="container">
                    <textarea
                        placeholder={ "日本語の文章を入力してください。" }
                        value={ self.input.clone() }
                        oninput={ ctx.link().callback(|e| Msg::Input(e)) }
                    >
                    </textarea>
                    {
                        if self.input.len() != 0 && words.len() > 0 {
                            html! {
                                <ol>
                                    { for words.iter().map(|text| { html! { <li>{ text }</li> } } ) }
                                </ol>
                            }
                        } else {
                            html! {}
                        }
                    }
                </main>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
