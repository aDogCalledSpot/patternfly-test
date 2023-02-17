use crate::example::ExamplePage;
use patternfly_yew::prelude::*;
use strum::{EnumMessage, IntoEnumIterator};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IconDescriptor(Icon);

impl TableRenderer for IconDescriptor {
    fn render(&self, column: ColumnIndex) -> Html {
        match column.index {
            0 => html!(self.0.as_html()),
            1 => html!(<code>{self.0.as_ref()}</code>),
            2 => html!(self
                .0
                .get_documentation()
                .map(Html::from)
                .unwrap_or_default()),
            _ => html!(),
        }
    }
}

#[function_component(Icons)]
pub fn icons() -> Html {
    let subtitle = html!(
        <div>
            <p>{"Sprinkle some icons into you application to make it look nice."}
            <Button variant={Variant::Link} label="PatternFly - Icons" icon={Icon::ExternalLinkAlt} align={Align::End} />
            </p>
        </div>
    );

    let entries = use_memo(
        |()| {
            let mut icons = Icon::iter().map(IconDescriptor).collect::<Vec<_>>();
            icons.sort_by(|a, b| a.0.as_ref().cmp(b.0.as_ref()));
            SharedTableModel::new(icons)
        },
        (),
    );

    let header = use_memo(
        |()| {
            html_nested!(
                <TableHeader>
                    <TableColumn/>
                    <TableColumn label="Name"/>
                    <TableColumn label="Description"/>
                </TableHeader>
            )
        },
        (),
    );

    html!(
        <ExamplePage title="Icons" {subtitle}>
            <Table<SharedTableModel<IconDescriptor>> header={(*header).clone()} entries={(*entries).clone()}/>
        </ExamplePage>
    )
}
