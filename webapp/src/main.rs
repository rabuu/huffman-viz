use huffman::{Node, Tree};
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

        let msg_input_changed = self
            .link
            .callback(|input: InputData| Msg::UpdateInput(input.value));

        let msg_create_tree = self.link.callback(|_| Msg::CreateTree);
        let msg_step = self.link.callback(|_| Msg::Step);
        let msg_build = self.link.callback(|_| Msg::Build);

        fn view_node(node: &Node<char>) -> Html {
            match node {
                Node::Tail {freq, val} => {
                    html!(
                        <div class="node tail">
                            {format!("{} ({})", freq, val)}
                        </div>
                    )
                }
                Node::Link {freq, left, right} => {
                    html!(
                        <div class="link-container">
                            <div class="node link">
                                {format!("{}", freq)}
                            </div>
                            {view_node(left)}
                            {view_node(right)}
                        </div>
                    )
                }
            }
        }

        html! {
            <div>
                <div id="controll-container">
                    <input type="text" placeholder="Type message here" oninput=msg_input_changed />
                    <button onclick=msg_create_tree>{ "Create Tree" }</button>
                    <div
                        style=match show_tree {
                            true => "display: block",
                            false => "display: none",
                        }
                    >
                        <button onclick=msg_step>{ "Step" }</button>
                        <button onclick=msg_build>{ "Build" }</button>
                    </div>
                </div>
                <div
                    style=match show_tree {
                        true => "display: flex",
                        false => "display: none",
                    }
                    id="tree-container"
                >
                    { match self.tree {
                        Some(ref tree) => html!({for tree.arena.iter().map(view_node)}),
                        None => html!(),
                    } }
                </div>
                <div id="debug-container">
                    { format!("input: {}", self.input) }
                <pre>{ match self.tree {
                    Some(ref tree) => format!("{:#?}", tree),
                    None => String::from(""),
                } }</pre>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
