use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

use super::table_context::{TableContext, TableContextVariant};

pub struct TableRow {
    table_context: TableContext
}

#[derive(Clone, PartialEq, Properties)]
pub struct TableRowProps {
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

impl Component for TableRow {
    type Message = ();
    type Properties = TableRowProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (context, _) = ctx.link()
            .context::<TableContext>(Callback::noop())
            .expect("No table context");

        TableRow { table_context: context }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut base_classes = vec!["text-inherit", "table-row", "align-middle", "border-b" , "border-solid", "border-zinc-500", "outline-0"];
        let class = ctx.props().class.clone();

        let variant = &self.table_context.variant;
        if let TableContextVariant::Body = variant {
            base_classes.append(&mut vec!["hover:bg-zinc-700"]);
        } else {
            base_classes.append(&mut vec!["bg-neutral-700"]);
        }

        html! {
            <tr class={classes!(base_classes, class)}>
                {ctx.props().children.clone()}
            </tr>
        }
    }
}