use gloo_console::log;
use yew::prelude::*;

#[derive(Debug)]
pub struct TestComponent {
    label: String,
}

pub enum Msg {
}


impl Component for TestComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        log!("TestComponent::create()");
        Self {
            label: String::from("Trunk Rendered Successfully")
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        log!("TestComponent::view()");
        html! {
            <h1 class="Component" style="text-align:center">
                { self.label.clone() }
            </h1>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        log!("TestComponent::rendered()");
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        log!("TestComponent::update()");
        // match msg {}
        false  // no rerender
    }
}
