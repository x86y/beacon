use crate::text_input;
use iced::{
    widget::{button, container},
    Color,
};
use iced::{Background, Vector};

pub struct BtnStyle;
pub struct TabStyle;
pub struct CanvasStyle;
pub struct ToolbarStyle;
pub struct SrcCellStyle;
pub struct ElapsedTimeStyle;
pub struct ErroredCellStyle;
pub struct ActiveTabStyle;
pub struct InputStyle;

impl iced::widget::button::StyleSheet for BtnStyle {
    type Style = iced::Theme;
    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: None,
            text_color: Color::WHITE,
            ..Default::default()
        }
    }
}
impl BtnStyle {
    pub fn theme() -> iced::theme::Button {
        iced::theme::Button::Custom(Box::from(BtnStyle))
    }
}

impl iced::widget::button::StyleSheet for TabStyle {
    type Style = iced::Theme;
    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb(
                56.0 / 255.0,
                56.0 / 255.0,
                56.0 / 255.0,
            ))),
            text_color: Color::WHITE,
            ..Default::default()
        }
    }
}
impl TabStyle {
    pub fn theme() -> iced::theme::Button {
        iced::theme::Button::Custom(Box::from(TabStyle))
    }
}

impl iced::widget::container::StyleSheet for ToolbarStyle {
    type Style = iced::Theme;
    fn appearance(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgb(
                24.0 / 255.0,
                24.0 / 255.0,
                24.0 / 255.0,
            ))),
            text_color: Some(Color::WHITE),
            ..Default::default()
        }
    }
}
impl ToolbarStyle {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(ToolbarStyle))
    }
}

impl iced::widget::container::StyleSheet for CanvasStyle {
    type Style = iced::Theme;
    fn appearance(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgb(
                12.0 / 255.0,
                12.0 / 255.0,
                12.0 / 255.0,
            ))),
            text_color: Some(Color::WHITE),
            ..Default::default()
        }
    }
}
impl CanvasStyle {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(CanvasStyle))
    }
}

impl SrcCellStyle {
    pub fn theme() -> iced::theme::Text {
        iced::theme::Text::Color(Color::from_rgba(0.3, 0.9, 0.3, 0.9))
    }
}

impl iced::widget::text_input::StyleSheet for InputStyle {
    type Style = iced::Theme;
    fn active(&self, _: &Self::Style) -> iced_style::text_input::Appearance {
        text_input::Appearance {
            border_color: Color::from_rgba(0.3, 0.3, 0.3, 1.0),
            background: Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 1.0)),
            border_radius: 0.0,
            border_width: 1.0,
        }
    }
    fn focused(&self, _: &Self::Style) -> iced_style::text_input::Appearance {
        text_input::Appearance {
            border_color: Color::from_rgba(0.3, 0.3, 0.3, 1.0),
            background: Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 1.0)),
            border_radius: 0.0,
            border_width: 1.0,
        }
    }
    fn placeholder_color(&self, _: &Self::Style) -> Color {
        Color::from_rgba(0.0, 0.0, 0.0, 1.0)
    }
    fn value_color(&self, _: &Self::Style) -> Color {
        Color::from_rgba(1.0, 1.0, 1.0, 0.95)
    }
    fn selection_color(&self, _: &Self::Style) -> Color {
        Color::from_rgba(0.2, 0.0, 0.5, 1.0)
    }
}
impl InputStyle {
    pub fn theme() -> iced::theme::TextInput {
        iced::theme::TextInput::Custom(Box::from(InputStyle))
    }
}

impl ActiveTabStyle {
    pub fn theme() -> iced::theme::Text {
        iced::theme::Text::Color(Color::from_rgba(0.1, 0.8, 0.1, 0.9))
    }
}
impl ElapsedTimeStyle {
    pub fn theme() -> iced::theme::Text {
        iced::theme::Text::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.2))
    }
}
impl ErroredCellStyle {
    pub fn theme() -> iced::theme::Text {
        iced::theme::Text::Color(Color::from_rgba(1.0, 0.0, 0.1, 0.95))
    }
}
