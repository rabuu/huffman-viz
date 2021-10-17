use huffman::{Node, Tree};
use yew::prelude::*;

enum Msg {
    UpdateInput(String),
    CreateTree,
    Step,
    Build,
    ShowCodeTable,
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
                    .step()
                    .unwrap_or(());
                true
            }
            Msg::Build => {
                self.tree
                    .as_mut()
                    .expect("Called build method on non-existent tree.")
                    .build();
                true
            }
            Msg::ShowCodeTable => {
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let some_tree = self.tree.is_some();

        let step_build_container_style = match some_tree && !self.tree.as_ref().unwrap().is_built() {
            true => "display: flex",
            false => "display: none",
        };

        let tree_container_style = match some_tree {
            true => "display: flex",
            false => "display: none",
        };

        let code_table_btn_container_style = match some_tree && self.tree.as_ref().unwrap().is_built() {
            true => "display: flex",
            false => "display: none",
        };

        let msg_input_changed = self
            .link
            .callback(|input: InputData| Msg::UpdateInput(input.value));

        let msg_create_tree = self.link.callback(|_| Msg::CreateTree);
        let msg_step = self.link.callback(|_| Msg::Step);
        let msg_build = self.link.callback(|_| Msg::Build);
        let msg_show_code_table = self.link.callback(|_| Msg::ShowCodeTable);

        let view_tree = match self.tree {
            Some(ref tree) => html!({for tree.get_arena().iter().map(view_node)}),
            None => html!(),
        };

        let view_tree_debug = match self.tree {
            Some(ref tree) => format!("{:#?}", tree),
            None => String::from(""),
        };

        fn view_node(node: &Node<char>) -> Html {
            match node {
                Node::Tail { freq, val } => {
                    html!(
                        <div class="node tail">
                            {format!("{} ({})", freq, val)}
                        </div>
                    )
                }
                Node::Link { freq, left, right } => {
                    html!(
                        <div class="link-container">
                            <div class="node link">
                                {format!("{}", freq)}
                            </div>
                            <div class="branches-container">
                                {view_node(left)}
                                {view_node(right)}
                            </div>
                        </div>
                    )
                }
            }
        }

        html! {
            <div>
                <h1>{"Huffman Visualizer"}</h1>
                <div id="controll-container">
                    <input type="text" placeholder="Type message here" oninput=msg_input_changed />
                    <button onclick=msg_create_tree>{ "Create Tree" }</button>
                    <div style=step_build_container_style id="step-build-container">
                        <button onclick=msg_step>{ "Step" }</button>
                        <button onclick=msg_build>{ "Build" }</button>
                    </div>
                    <div style=code_table_btn_container_style id="code-table-btn-container">
                        <button onclick=msg_show_code_table>{ "Show Code Table" }</button>
                    </div>
                </div>
                <div style=tree_container_style id="tree-container">
                    { view_tree }
                </div>
                <div id="debug-container">
                    { format!("input: {}", self.input) }
                <pre>{ view_tree_debug }</pre>
                </div>
                <div id="src-code">
                    <a href="https://gitlab.com/rabuu/huffman-viz">{"source code"}</a>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
