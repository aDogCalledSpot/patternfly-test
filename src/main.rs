use patternfly_yew::{next, prelude::*};
use yew::prelude::*;
use std::rc::Rc;

#[function_component(App)]
pub fn add_channel() -> Html {
    html! {
        <Form>
            <FormGroup label="Name">
                <next::TextInput
                required=true/>
            </FormGroup>
            <FormGroup>
                <ComponentSelector/>
            </FormGroup>
            <ActionGroup>
                <Button variant={ButtonVariant::Primary} label="Submit" r#type={ButtonType::Submit}/>
            </ActionGroup>
        </Form>
    }
}


#[function_component(ComponentSelector)]
fn app() -> Html {
    let header = html_nested! {
        <TreeTableHeader>
            <TableColumn label="foo" width={ColumnWidth::Percent(40)}/>
            <TableColumn label="L1" width={ColumnWidth::Percent(20)}/>
            <TableColumn label="L2" width={ColumnWidth::Percent(20)}/>
            <TableColumn label="L3" width={ColumnWidth::Percent(20)}/>
        </TreeTableHeader>
    };

    #[derive(PartialEq)]
    struct Root {
        root: Rc<Node>,
    }

    #[derive(PartialEq)]
    enum Node {
        Branch(Vec<String>, Vec<Rc<Node>>),
        Leaf(Vec<String>)
    }

    impl TreeTableModel for Root {
        fn children(&self) -> Vec<Rc<dyn TreeNode>> {
            self.root.children()
        }
    }

    impl TreeNode for Node {
        fn render_main(&self) -> Cell {
            match self {
                Self::Branch(name, _) => html!({name.join(" / ")}),
                Self::Leaf(name) => html!({name.join(" / ")}),
            }.into()
        }

        fn render_cell(&self, context: CellContext) -> Cell {
            let name = match self {
                Self::Branch(name, _) | Self::Leaf(name) => name,
            };

            // quick alternative to: match context.column { 0=> {}, … }
            name.get(context.column)
                .map(Html::from)
                .unwrap_or_default()
                .into()
        }

        fn children(&self) -> Vec<Rc<dyn TreeNode>> {
            match self {
                Self::Branch(_, children) => children.iter().map(|c|c.clone() as Rc<dyn TreeNode>).collect(),
                Self::Leaf(_) => vec![],
            }
        }
    }

    let mut root = vec![];
    for a in ["I", "II", "III"] {
        let mut folders = vec![];
        for b in [1,2,3] {
            let mut leaves = vec![];
            for c in ["a", "b", "c"] {
                leaves.push(Rc::new(Node::Leaf(vec![a.to_string(), b.to_string(), c.to_string()])));
            }
            folders.push(Rc::new(Node::Branch(vec![a.to_string(), b.to_string()], leaves)));
        }
        root.push(Rc::new(Node::Branch(vec![a.to_string()], folders)));
    }

    let root = Rc::new(Root {root: Rc::new(Node::Branch(vec![], root))});

    html!{
        <TreeTable<Root>
            mode={TreeTableMode::Compact}
            header={header}
            model={root}
        />
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
