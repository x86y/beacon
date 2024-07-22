use crate::widgets::text_input;
use crate::widgets::text_input::Status;
use iced::widget::{button, container, scrollable};
use iced::{Border, Color, Theme};

pub fn btnstyle(_a: &Theme, _b: button::Status) -> button::Style {
    button::Style {
        background: None,
        text_color: Color::WHITE,
        ..Default::default()
    }
}

pub fn outstyle(_a: &Theme, _b: button::Status) -> button::Style {
    button::Style {
        text_color: Color::WHITE,
        ..Default::default()
    }
}

pub fn toolbarstyle(_a: &Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(Color::from_rgb(
            24.0 / 255.0,
            24.0 / 255.0,
            24.0 / 255.0,
        ))),
        text_color: Some(Color::WHITE),
        ..Default::default()
    }
}
pub fn canvasstyle(_a: &Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(Color::from_rgb(
            12.0 / 255.0,
            12.0 / 255.0,
            12.0 / 255.0,
        ))),
        text_color: Some(Color::WHITE),
        ..Default::default()
    }
}
pub fn inputstyle(_a: &Theme, _b: Status) -> text_input::Style {
    text_input::Style {
        background: iced::Background::Color(Color::from_rgb(
            12.0 / 255.0,
            12.0 / 255.0,
            12.0 / 255.0,
        )),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        icon: Color::BLACK,
        placeholder: Color::WHITE,
        value: Color::from_rgb(1.0, 1.0, 0.0),
        selection: Color::WHITE,
    }
}

pub fn scrollbarstyle() -> scrollable::Style {
    scrollable::Style {
        container: container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(
                24.0 / 255.0,
                24.0 / 255.0,
                24.0 / 255.0,
            ))),
            text_color: Some(Color::WHITE),
            ..Default::default()
        },
        vertical_rail: scrollable::Rail {
            background: None,
            border: Border {
                color: Color::BLACK,
                width: 0.0,
                radius: 0.0.into(),
            },
            scroller: scrollable::Scroller {
                color: Color::BLACK,
                border: Border {
                    color: Color::BLACK,
                    width: 0.0,
                    radius: 0.0.into(),
                },
            },
        },
        horizontal_rail: scrollable::Rail {
            background: None,
            border: Border {
                color: Color::BLACK,
                width: 0.0,
                radius: 0.0.into(),
            },
            scroller: scrollable::Scroller {
                color: Color::BLACK,
                border: Border {
                    color: Color::BLACK,
                    width: 0.0,
                    radius: 0.0.into(),
                },
            },
        },
        gap: None,
    }
}
