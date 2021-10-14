use huffman::Tree;
use yew::prelude::*;

enum Msg {
    UpdateInput(String),
    CreateTree,
    Step,
    Build,
}

struct App {
    link: ComponentLink<Self>,
    input: String,
    tree: Option<Tree<char>>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            input: String::from(""),
            tree: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateInput(input) => {
                self.input = input;
                false
            }
            Msg::CreateTree => {
                let tree = Tree::new_from_string(self.input.clone());
                self.tree = Some(tree);
                true
            }
            Msg::Step => {
                self.tree
                    .as_mut()
                    .expect("Called step method on non-existent tree.")
                    .step();
                true
            }
            Msg::Build => {
                self.tree
                    .as_mut()
                    .expect("Called build method on non-existent tree.")
                    .build();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let show_tree = self.tree.is_some();

        let input_changed = self
            .link
            .callback(|input: InputData| Msg::UpdateInput(input.value));

        let create_tree = self.link.callback(|_| Msg::CreateTree);
        let step = self.link.callback(|_| Msg::Step);
        let build = self.link.callback(|_| Msg::Build);

        html! {
            <div>
                <div>
                    <input type="text" placeholder="Type message here" oninput=input_changed />
                    <button onclick=create_tree>{ "Create Tree" }</button>
                    <div style=match show_tree {
                        true => "display: block",
                        false => "display: none",
                    }>
                        <button onclick=step>{ "Step" }</button>
                        <button onclick=build>{ "Build" }</button>
                    </div>
                </div>
                <div style=match show_tree {
                    true => "visibility: visible",
                    false => "visibility: hidden",
                } id="tree-container">
                    { match self.tree { Some(ref tree) => format!("{:#?}", tree), None => "".to_string() } }
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
