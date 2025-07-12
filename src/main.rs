use iced::widget::{button, column, row, text, text_input};
use iced::{Alignment, Color, Element, Event, Length, Task as Command, Theme, event};
use iced_layershell::Application;
use iced_layershell::reexport::{Anchor, KeyboardInteractivity, Layer};
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};
use iced_layershell::to_layer_message;

struct Counter {
    value: i32,
    text: String,
}

#[derive(Debug, Clone, Copy)]
enum WindowDirection {
    Top,
    Left,
    Right,
    Bottom,
}

// Because new iced delete the custom command, so now we make a macro crate to generate
// the Command
#[to_layer_message]
#[derive(Debug, Clone)]
#[doc = "Some docs"]
enum Message {
    IncrementPressed,
    DecrementPressed,
    TextInput(String),
    Direction(WindowDirection),
    IcedEvent(Event),
}

impl Application for Counter {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = iced::executor::Default;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                value: 0,
                text: "hello, write something here".to_string(),
            },
            Command::none(),
        )
    }

    fn namespace(&self) -> String {
        String::from("Counter - Iced")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        event::listen().map(Message::IcedEvent)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::IcedEvent(event) => {
                println!("hello {event:?}");
                Command::none()
            }
            Message::IncrementPressed => {
                self.value += 1;
                Command::none()
            }
            Message::DecrementPressed => {
                self.value -= 1;
                Command::none()
            }
            Message::TextInput(text) => {
                self.text = text;
                Command::none()
            }

            Message::Direction(direction) => match direction {
                WindowDirection::Left => Command::done(Message::AnchorSizeChange(
                    Anchor::Left | Anchor::Top | Anchor::Bottom,
                    (200, 0),
                )),
                WindowDirection::Right => Command::done(Message::AnchorSizeChange(
                    Anchor::Right | Anchor::Top | Anchor::Bottom,
                    (200, 0),
                )),
                WindowDirection::Bottom => Command::done(Message::AnchorSizeChange(
                    Anchor::Bottom | Anchor::Left | Anchor::Right,
                    (0, 200),
                )),
                WindowDirection::Top => Command::done(Message::AnchorSizeChange(
                    Anchor::Top | Anchor::Left | Anchor::Right,
                    (0, 200),
                )),
            },
            _ => unreachable!(),
        }
    }

    fn view(&self) -> Element<Message> {
        let center = column![
            button("Increment").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            button("Decrement").on_press(Message::DecrementPressed)
        ]
        .align_x(Alignment::Center)
        .padding(20)
        .width(Length::Fill)
        .height(Length::Fill);
        row![
            button("left")
                .on_press(Message::Direction(WindowDirection::Left))
                .height(Length::Fill),
            column![
                button("top")
                    .on_press(Message::Direction(WindowDirection::Top))
                    .width(Length::Fill),
                center,
                text_input("hello", &self.text)
                    .on_input(Message::TextInput)
                    .padding(10),
                button("bottom")
                    .on_press(Message::Direction(WindowDirection::Bottom))
                    .width(Length::Fill),
            ]
            .width(Length::Fill),
            button("right")
                .on_press(Message::Direction(WindowDirection::Right))
                .height(Length::Fill),
        ]
        .padding(20)
        .spacing(10)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn style(&self, theme: &Self::Theme) -> iced_layershell::Appearance {
        use iced_layershell::Appearance;
        Appearance {
            background_color: Color::from_rgb(255.0, 255.0, 0.0),
            text_color: theme.palette().text,
        }
    }
}
pub fn main() -> Result<(), iced_layershell::Error> {
    let binded_output_name = std::env::args().nth(1);
    let start_mode = match binded_output_name {
        Some(output) => StartMode::TargetScreen(output),
        None => StartMode::Active,
    };

    Counter::run(Settings {
        layer_settings: LayerShellSettings {
            size: Some((0, 200)),
            exclusive_zone: 200,
            anchor: Anchor::Bottom | Anchor::Left | Anchor::Right,
            start_mode: StartMode::Active,
            events_transparent: false,
            keyboard_interactivity: KeyboardInteractivity::OnDemand,
            layer: iced_layershell::reexport::Layer::Overlay,
            margin: (0, 0, 0, 0),
        },
        ..Default::default()
    })
}
