use iced::{
    widget::{button, row, Container},
    Element, Font,
};

use crate::{
    utils::{macros::bqn386, HistoryMap},
    Message,
};

pub fn tab_view<'a>(outs: &HistoryMap, at: usize) -> Element<'a, Message> {
    let mut keys: Vec<_> = outs.0.keys().cloned().collect();
    keys.sort();
    let tabs: iced::widget::Row<Message> = row({
        keys.iter()
            .map(|i| {
                Container::new(
                    button(if *i == at {
                        bqn386!(format!("[{i}]"))
                    } else {
                        bqn386!(format!("{i}"))
                    })
                    .style(|theme, status| match status {
                        button::Status::Active => button::Style {
                            background: Some(iced::Background::Color(iced::Color::from_rgb(
                                56.0 / 255.0,
                                56.0 / 255.0,
                                56.0 / 255.0,
                            ))),
                            text_color: iced::Color::WHITE,
                            ..Default::default()
                        },
                        button::Status::Pressed => button::Style {
                            background: Some(iced::Background::Color(iced::Color::from_rgb(
                                12.0 / 255.0,
                                12.0 / 255.0,
                                12.0 / 255.0,
                            ))),
                            text_color: iced::Color::WHITE,
                            ..Default::default()
                        },
                        _ => button::Style {
                            background: Some(iced::Background::Color(iced::Color::from_rgb(
                                12.0 / 255.0,
                                12.0 / 255.0,
                                12.0 / 255.0,
                            ))),
                            text_color: iced::Color::WHITE,
                            ..Default::default()
                        },
                    })
                    .on_press(Message::TabChanged(*i)),
                )
                .into()
            })
            .collect::<Vec<_>>()
    });
    let new_tab_btn = button(bqn386!("+"))
        .on_press(Message::TabCreate)
        .style(|theme, status| match status {
            button::Status::Active => button::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(
                    56.0 / 255.0,
                    56.0 / 255.0,
                    56.0 / 255.0,
                ))),
                text_color: iced::Color::WHITE,
                ..Default::default()
            },
            _ => button::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(
                    12.0 / 255.0,
                    12.0 / 255.0,
                    12.0 / 255.0,
                ))),
                text_color: iced::Color::WHITE,
                ..Default::default()
            },
        });
    row![tabs, new_tab_btn].spacing(0).spacing(20).into()
}
