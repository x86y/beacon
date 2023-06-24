use crate::text_input;
use iced::widget::{button, container};
use iced::Background;
use iced::Color;

pub struct BtnStyle;
pub struct TabStyle;
pub struct CanvasStyle;
pub struct ToolbarStyle;
pub struct SrcCellStyle;
pub struct ElapsedTimeStyle;
pub struct ErroredCellStyle;
pub struct ActiveTabStyle;
pub struct InputStyle;
pub struct TooltipStyle;
pub struct ScrollbarStyle;

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

impl text_input::StyleSheet for InputStyle {
    type Style = iced::Theme;
    fn disabled_color(&self, style: &Self::Style) -> Color {
        Color::BLACK
    }
    fn disabled(&self, style: &Self::Style) -> iced_style::text_input::Appearance {
        text_input::Appearance {
            border_color: Color::from_rgba(0.3, 0.3, 0.3, 0.0),
            background: Background::Color(Color::from_rgba(
                12.0 / 255.0,
                12.0 / 255.0,
                12.0 / 255.0,
                1.0,
            )),
            border_radius: 0.0.into(),
            border_width: 1.0,
            icon_color: Color::BLACK,
        }
    }
    fn active(&self, _: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            border_color: Color::from_rgba(0.3, 0.3, 0.3, 0.0),
            background: Background::Color(Color::from_rgba(
                12.0 / 255.0,
                12.0 / 255.0,
                12.0 / 255.0,
                1.0,
            )),
            border_radius: 0.0.into(),
            border_width: 1.0,
            icon_color: Color::BLACK,
        }
    }
    fn focused(&self, _: &Self::Style) -> iced_style::text_input::Appearance {
        text_input::Appearance {
            border_color: Color::from_rgba(0.3, 0.3, 0.3, 0.0),
            background: Background::Color(Color::from_rgba(
                12.0 / 255.0,
                12.0 / 255.0,
                12.0 / 255.0,
                1.0,
            )),
            border_radius: 0.0.into(),
            border_width: 1.0,
            icon_color: Color::BLACK,
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

impl iced::widget::container::StyleSheet for TooltipStyle {
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
impl TooltipStyle {
    pub fn theme() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(TooltipStyle))
    }
}

impl iced::widget::scrollable::StyleSheet for ScrollbarStyle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> iced_style::scrollable::Scrollbar {
        iced_style::scrollable::Scrollbar {
            background: Some(iced::Background::Color(Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            })),
            border_radius: [0.0, 0.0, 0.0, 0.0].into(),
            border_width: 4.0,
            border_color: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            scroller: iced_style::scrollable::Scroller {
                color: Color::WHITE,
                border_radius: [0.0, 0.0, 0.0, 0.0].into(),
                border_width: 4.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(
        &self,
        style: &Self::Style,
        is_mouse_over_scrollbar: bool,
    ) -> iced_style::scrollable::Scrollbar {
        iced_style::scrollable::Scrollbar {
            background: Some(iced::Background::Color(Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            })),
            border_radius: [0.0, 0.0, 0.0, 0.0].into(),
            border_width: 4.0,
            border_color: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            scroller: iced_style::scrollable::Scroller {
                color: Color::WHITE,
                border_radius: [0.0, 0.0, 0.0, 0.0].into(),
                border_width: 4.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }
}
impl ScrollbarStyle {
    pub fn theme() -> iced::theme::Scrollable {
        iced::theme::Scrollable::Custom(Box::from(ScrollbarStyle))
    }
}
