use iced::{
    widget::{button, container, text, tooltip},
    Element,
};
use iced_core::{text::LineHeight, Font};
use once_cell::sync::Lazy;

use crate::{
    docs::content::glyph_to_documentation,
    styles::{BtnStyle, TooltipStyle},
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
                            .style(BtnStyle::theme())
                            .on_press(Message::ToolbarClick(glyph.to_string())),
                        glyph_to_documentation(*glyph),
                        tooltip::Position::FollowCursor,
                    )
                    .font(Font::with_name("BQN386 Unicode"))
                    .style(TooltipStyle::theme()),
                )
            })
            .spacing(1),
    )
    .into()
}
