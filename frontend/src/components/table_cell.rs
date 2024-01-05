use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

use super::table_context::{TableContext, TableContextVariant};

pub struct TableCell {
    table_context: TableContext
}

#[derive(Clone, PartialEq, Properties)]
pub struct TableCellProps {
    pub children: Children,
    #[prop_or(TableTextAlign::Left)]
    pub align: TableTextAlign,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub checkbox: bool,
}

#[derive(Clone, PartialEq)]
pub enum TableTextAlign {
    Left,
    Center,
    Right,
    Justify
}

impl Component for TableCell {
    type Message = ();
    type Properties = TableCellProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (context, _) = ctx.link()
            .context::<TableContext>(Callback::noop())
            .expect("No table context");

        TableCell { table_context: context }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut shared_classes = vec!["table-cell", "border-b-1", "border-solid"];
        let class = ctx.props().class.clone();
        let variant = &self.table_context.variant;

        if ctx.props().checkbox {
            shared_classes.append(&mut vec!["w-[48px]", "p-0", "pl-1"])
        } else {
            shared_classes.append(&mut vec!["p-4"])
        }

        match &ctx.props().align {
            TableTextAlign::Left => { shared_classes.append(&mut vec!["text-left"]) },
            TableTextAlign::Center => { shared_classes.append(&mut vec!["text-center"]) },
            TableTextAlign::Right => { shared_classes.append(&mut vec!["text-right", "flex-row-reverse"]) },
            TableTextAlign::Justify => { shared_classes.append(&mut vec!["text-justify"]) },
        }

        if let TableContextVariant::Head = variant {
            html! {
                <th scope="col" class={classes!(shared_classes, "font-medium", "leading-6", class)}>
                    {ctx.props().children.clone()}
                </th>
            }
        } else {
            html! {
                <td class={classes!(shared_classes, class)}>
                    {ctx.props().children.clone()}
                </td>
            }
        }
    }
}