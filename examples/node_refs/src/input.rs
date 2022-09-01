use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum Msg {
    Hover,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub placeholder: AttrValue,
}
pub struct InputComponent {
    node_ref: NodeRef
}

impl Component for InputComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Hover => {
                if let Some(input) = self.node_ref.cast::<HtmlInputElement>() {
                    input.focus().ok();
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let placeholder = ctx.props().placeholder.clone();
        html! {
            <input
                ref={self.node_ref.clone()}
                type="text"
                class="input-component"
                placeholder={placeholder}
                onmouseover={ctx.link().callback(|_| Msg::Hover)}
            />
        }
    }
}
