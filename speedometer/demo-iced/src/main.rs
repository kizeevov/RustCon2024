use iced::widget::{column, Column};
use iced::{Center, Length};

use speedometer::speedometer;
use theme::Theme as DashboardTheme;

mod speedometer;
mod theme;

const MAX_SPEED: u32 = 200;
const MIN_SPEED: u32 = 0;

pub fn main() -> iced::Result {
    iced::application("Iced Dashboard", Dashboard::update, Dashboard::view)
        .theme(Dashboard::theme)
        .antialiasing(true)
        .run()
}

#[derive(Default)]
struct Dashboard {
    speed: u32,
    theme: DashboardTheme,
}

impl Dashboard {
    fn update(&mut self, message: Message) {
        match message {
            Message::SpeedChanged(value) => {
                self.speed = value;
            }
            Message::None => {}
        }
    }

    fn view(&self) -> Column<Message> {
        column![speedometer(MIN_SPEED, MAX_SPEED, self.speed as u32)
            .width(Length::Fill)
            .height(Length::Fill)
            .view()
            .map(|_| Message::None),]
        .padding(20)
        .align_x(Center)
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::custom_with_fn(String::from("Custom"), self.theme.palette(), |_| {
            self.theme.extended_palette()
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    None,
    #[allow(dead_code)]
    SpeedChanged(u32),
}
