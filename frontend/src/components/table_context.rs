use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

#[derive(Clone, Debug, PartialEq)]
pub struct TableContext {
    pub variant: TableContextVariant
}

#[derive(Clone, PartialEq, Properties)]
pub struct TableContextProps {
    pub children: Children,
    pub context: TableContext
}

#[derive(Clone, Debug, PartialEq)]
pub enum TableContextVariant {
    Head,
    Body,
    Footer,
}

impl Component for TableContext {
    type Message = ();
    type Properties = TableContextProps;

    fn create(_ctx: &Context<Self>) -> Self {
        TableContext { variant: TableContextVariant::Body }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let context = ctx.props().context.clone();

        html! {
            <ContextProvider<TableContext> context={context}>
                {ctx.props().children.clone()}
            </ContextProvider<TableContext>>
        }
    }
}