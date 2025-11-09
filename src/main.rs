use gpui::{
    AnyElement, AppContext, Application, Bounds, Div, ElementId, EventEmitter, ParentElement,
    Render, SharedString, Stateful, StyleRefinement, WindowBounds, WindowOptions, div, prelude::*, px, size
};

struct MyApp {
    count: i32
}

#[derive(IntoElement)]
struct MyButton {
    base: Stateful<Div>,
    children: Vec<AnyElement>,
    label: Option<SharedString>,
}

impl MyButton {
    fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: div().id(id.into()),
            children: vec![],
            label: Default::default()

        }
    }
    fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl From<MyButton> for AnyElement {
    fn from(value: MyButton) -> Self {
        value.into_any_element()
    }
}

impl Styled for MyButton {
    fn style(&mut self) ->  &mut StyleRefinement {
        self.base.style()
    }
}
impl ParentElement for MyButton {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl InteractiveElement for MyButton {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for MyButton {}

impl RenderOnce for MyButton {
    fn render(self, window: &mut gpui::Window, cx: &mut gpui::App) -> impl IntoElement {
        self.base
            .p_2()
            .bg(gpui::black())
            .border_2()
            .border_color(gpui::white())
            .when_some(self.label, |this, label| {
                this
                    .text_color(gpui::white())
                    .child(label)
            })
            .when(!self.children.is_empty(), |this| {
                this.child(div().children(self.children))
            })
    }
}

impl Render for MyApp {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .flex()
            .items_center()
            .justify_center()
            .w_full()
            .h_full()
            .bg(gpui::black())
            .child(div()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap_6()
                .child(div()
                    .flex()
                    .flex_row()
                    .gap_6()
                    .child(MyButton::new("inc_btn")
                        .label("incrment")
                        .on_click(cx.listener(|btn, event, win, cx| {
                            cx.emit(Command::Incrment);
                        })))
                    .child(MyButton::new("dec_btn")
                        .label("decrment")
                        .on_click(cx.listener(|btn, event, win, cx| {
                            cx.emit(Command::Decrment);
                        }))))
                .child(div().text_color(gpui::white()).child(self.count.to_string())))
    }
}

#[derive(Debug)]
enum Command {
    Incrment,
    Decrment
}

impl EventEmitter<Command> for MyApp {}

fn main() {
    Application::new()
        .run(|cx| {
        let entity = cx.new(|_| MyApp { count:0 });
        _ = cx.subscribe(&entity, |entity, event, cx| {
            println!("on_subscribe: {:?}", event);
            entity.update(cx, |entity, cx| {
                match event {
                    Command::Incrment => {
                        entity.count += 1;
                    }
                    Command::Decrment => {
                        entity.count -= 1;
                    }
                }
            })
        }).detach(); // here, this is Future<T>

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(800.0), px(600.0)),
                    cx,
                ))),
                ..Default::default()
            },
            |window, cx| {
                entity
            },
        )
        .unwrap();
    });
}
