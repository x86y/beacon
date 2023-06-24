use iced::{
    widget::{button, row, text, Container},
    Element,
};
use iced_core::{text::LineHeight, Font};

use crate::{
    styles::{ActiveTabStyle, TabStyle},
    utils::{macros::bqn386, HistoryMap},
    Message,
};

pub fn tab_view<'a>(outs: &HistoryMap, at: usize) -> Element<'a, Message> {
    let tabs: iced::widget::Row<Message> = row({
        outs.0
            .keys()
            // .sorted()
            .map(|i| {
                Container::new(
                    button(if *i == at {
                        bqn386!(format!("{i}")).style(ActiveTabStyle::theme())
                    } else {
                        bqn386!(format!("{i}"))
                    })
                    .on_press(Message::TabChanged(*i))
                    .style(TabStyle::theme()),
                )
                .into()
            })
            .collect()
    });
    let new_tab_btn = button(bqn386!("+"))
        .on_press(Message::TabCreate)
        .style(TabStyle::theme());
    row![tabs, new_tab_btn].spacing(0).spacing(20).into()
}
