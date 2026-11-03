use xenia_kit::{
    html,
    ux::{Button, ButtonTone, Card, CardBody, Hx, Swap, Toast, ToastKind},
    Fragment,
};

pub async fn get() -> Fragment {
    let button = Button {
        tone: Some(ButtonTone::Primary),
        hx: Hx {
            get: Some("/api/tabs/hello-world/fragment".into()),
            target: Some("closest [data-pane-panel]".into()),
            swap: Some(Swap::OuterHtml),
            ..Hx::default()
        },
        ..Button::new("Say hello")
    };
    let card = Card {
        body: CardBody::Paragraph("Your first Xenia cartridge is at home in the crown.".into()),
        footer: vec![button],
        ..Card::default()
    };

    Fragment::new(
        "hello-world",
        html! {
            (card)
        },
    )
    .with_toast(Toast::new(ToastKind::Success, "Hello from Xenia"))
}
