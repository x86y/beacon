use std::collections::HashMap;

use crate::utils::macros::bqn386;
use crate::utils::EvalCell;
use crate::widgets::text_input;
use crate::{styles::*, INPUT_ID};
use crate::{Message, SCROLL_ID};
use iced::alignment::{self, Alignment};
use iced::widget::{pane_grid, svg};
use iced::{
    color, theme,
    widget::{button, column, container, row, scrollable, text, Column, Container},
    Element, Length,
};

use iced_core::{text::LineHeight, Font};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Pane {
    id: usize,
    pub is_pinned: bool,
}

impl Pane {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            is_pinned: false,
        }
    }
}

pub fn view_pane<'a>(
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    input_value: &'a HashMap<usize, String>,
    outs: Option<&'a Vec<EvalCell>>,
    idx: usize,
) -> Element<'a, Message> {
    let inp = text_input::text_input(
        "",
        &input_value
            .get(&unsafe { std::mem::transmute::<_, usize>(pane) })
            .unwrap_or(&String::new()),
    )
    .padding(15)
    .style(InputStyle::theme())
    .size(18)
    .font(Font::with_name("BQN386 Unicode"))
    .on_submit(Message::RunInput)
    .on_input(move |s| {
        Message::InputChanged(s.clone(), unsafe { std::mem::transmute::<_, usize>(pane) })
    })
    .id(INPUT_ID.clone());
    let out_cells: Column<_> = column(
        outs.unwrap_or(&vec![])
            .iter()
            .map(|txt| {
                let mut res = txt.res.to_string();
                let mut did_error = false;
                if txt.res.starts_with("CBQN error:") {
                    res = res.replace("CBQN error: ", "");
                    did_error = true;
                }
                let mut v = vec![
                    button(bqn386!(" ".to_string() + &txt.src).style(SrcCellStyle::theme()))
                        .on_press(Message::FillInput(txt.src.to_string()))
                        .style(BtnStyle::theme())
                        .into(),
                    button(bqn386!(res.clone()).style(if did_error {
                        ErroredCellStyle::theme()
                    } else {
                        Default::default()
                    }))
                    .on_press(Message::FillInput(res))
                    .style(BtnStyle::theme())
                    .into(),
                ];
                v.push(
                    bqn386!(format!("{}ms", txt.time.as_millis()))
                        .size(12)
                        .style(ElapsedTimeStyle::theme())
                        .into(),
                );
                Container::new(column(v))
                    .width(Length::Fill)
                    .style(CanvasStyle::theme())
                    .into()
            })
            .collect::<Vec<Element<_>>>(),
    )
    .spacing(8);
    let handle = |n| svg::Handle::from_path(format!(
            "{}/assets/{n}.svg",
            env!("CARGO_MANIFEST_DIR")
        ));

    let button = |label, message| {
        button( svg(handle(label)).width(Length::Fill).height(Length::Fill).style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                    color: Some(color!(0xffffff)),
                })))
        .padding(2)
        .style(TransparentBtn::theme())
        .on_press(message)
    };


    let mut controls = row![
        button("horizontal", Message::Split(pane_grid::Axis::Horizontal, pane),),
        button("vertical", Message::Split(pane_grid::Axis::Vertical, pane),)
    ]
    .spacing(5);
    if total_panes > 1 && !is_pinned {
        controls = controls.push(button("cross", Message::Close(pane)));
    }

    let content = column![
        controls,
        scrollable(out_cells)
            .height(Length::Fill)
            .style(ScrollbarStyle::theme())
            .id(SCROLL_ID.clone()),
        inp
    ]
    .width(Length::Fill)
    .spacing(10)
    .align_items(Alignment::Center);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(5)
        .center_y()
        .into()
}
