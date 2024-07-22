use iced::{
    widget::{button, container, markdown, text, tooltip},
    Element,
};
use iced_core::{text::LineHeight, Font};
use once_cell::sync::Lazy;

use crate::{
    docs::content::glyph_to_documentation,
    styles::{btnstyle, toolbarstyle},
    utils::macros::bqn386,
    widgets::wrap::Wrap,
    Message,
};

static GLYPHS: Lazy<[char; 64]> = Lazy::new(|| {
    [
        '+', '¨', '⊸', '⊑', '´', '∾', '×', '-', '≠', '∘', '˜', '=', '/', '<', '↕', '⥊', '⊢', '⟜',
        '⊏', '≡', '∧', '˘', '!', '>', '⌽', '↓', '¬', '↑', '∨', '`', '◶', '⍟', '⌜', '⊣', '⌾', '⌈',
        '⋈', '⊔', '⌊', '»', '⊐', '∊', '○', '≤', '|', '≢', '⍉', '÷', '≍', '˝', '⁼', '«', '≥', '˙',
        '⍋', '⍷', '⋆', '⊘', '⎉', '⚇', '⊒', '√', '⍒', '⎊',
    ]
});

pub fn toolbar_view<'a>() -> Element<'a, Message> {
    container(
        GLYPHS
            .iter()
            .fold(Wrap::new(), |wrap, glyph| {
                wrap.push(
                    tooltip(
                        button(bqn386!(glyph))
                            .style(btnstyle)
                            .on_press(Message::ToolbarClick(glyph.to_string())),
                        markdown(
                            markdown::parse(
                                glyph_to_documentation(*glyph),
                                iced::theme::Palette::DRACULA,
                            ),
                            markdown::Settings::default(),
                            (),
                        ),
                        tooltip::Position::FollowCursor,
                    )
                    .style(toolbarstyle),
                )
            })
            .spacing(1.0),
    )
    .into()
}
