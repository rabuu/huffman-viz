use yew::prelude::*;

enum Msg {
    UpdateInput(String),
}

struct App {
    link: ComponentLink<Self>,
    input: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            input: String::from(""),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateInput(input) => {
                self.input = input;
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let oninput = self.link.callback(|input: InputData| Msg::UpdateInput(input.value));
        html! {
            <div>
                <input type="text" placeholder="Type message here" oninput=oninput />
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
