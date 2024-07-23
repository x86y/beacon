use iced::{
    widget::{
        button, markdown,
        tooltip,
    },
    Element, Font, Padding,
};
use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::{
    docs::content::glyph_to_documentation, styles::btnstyle, utils::macros::bqn386,
    widgets::wrap::Wrap, Message,
};

static GLYPHS: Lazy<[char; 64]> = Lazy::new(|| {
    [
        '+', '¨', '⊸', '⊑', '´', '∾', '×', '-', '≠', '∘', '˜', '=', '/', '<', '↕', '⥊', '⊢', '⟜',
        '⊏', '≡', '∧', '˘', '!', '>', '⌽', '↓', '¬', '↑', '∨', '`', '◶', '⍟', '⌜', '⊣', '⌾', '⌈',
        '⋈', '⊔', '⌊', '»', '⊐', '∊', '○', '≤', '|', '≢', '⍉', '÷', '≍', '˝', '⁼', '«', '≥', '˙',
        '⍋', '⍷', '⋆', '⊘', '⎉', '⚇', '⊒', '√', '⍒', '⎊',
    ]
});

pub struct Toolbar {
    items: HashMap<char, Vec<markdown::Item>>,
}

impl Toolbar {
    pub fn new() -> Self {
        let items: HashMap<char, Vec<markdown::Item>> = GLYPHS
            .iter()
            .map(|&glyph| (glyph, parse_glyph_documentation(glyph)))
            .collect();

        Self { items }
    }

    pub fn view(&self) -> Element<'_, Message> {
        Wrap::with_elements(
            GLYPHS
                .iter()
                .map(|&glyph| {
                    tooltip(
                        button(bqn386!(glyph))
                            .style(btnstyle)
                            .on_press(Message::ToolbarClick(glyph.to_string())),
                        iced::widget::container(markdown(
                            &self.items[&glyph],
                            markdown::Settings::default(),
                            |url| Message::TabNext,
                        ))
                        .style(|t| iced::widget::container::Style {
                            text_color: Some(iced::Color::BLACK),
                            background: Some(iced::Background::Color(iced::Color::WHITE)),
                            border: iced::border::rounded(4),
                            shadow: Default::default(),
                        })
                        .padding(Padding::new(4.0))
                        .width(400.0)
                        .max_height(1000.0),
                        tooltip::Position::FollowCursor,
                    )
                    .into()
                })
                .collect(),
        )
        .into()
    }
}

fn parse_glyph_documentation(glyph: char) -> Vec<markdown::Item> {
    markdown::parse(glyph_to_documentation(glyph), iced::theme::Palette::DRACULA).collect()
}

