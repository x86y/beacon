#![allow(dead_code, unused_variables)]

use iced::widget::pane_grid::{self, PaneGrid};
use iced::widget::responsive;
use iced::{
    event::{self, Event},
    keyboard::{self, Modifiers},
    subscription,
    widget::{column, container, scrollable, text},
    window, Application, Command, Element, Length, Settings, Subscription, Theme,
};
use iced_runtime::font::load;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::time::Instant;
use styles::CanvasStyle;
use views::tabs::tab_view;
// use tracing::{event as e, info, instrument, Level};

mod docs;
mod save;
mod styles;
mod utils;
mod views;
mod widgets;
use crate::save::*;
use crate::utils::{EvalCell, Ty};
use crate::widgets::text_input;
#[cfg(feature = "k")]
use utils::keval;
use utils::{truncate, HistoryMap, REPL};
use views::pane::{view_pane, Pane};
use views::toolbar::toolbar_view;

pub static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);
static SCROLL_ID: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);

pub fn main() -> iced::Result {
    // tracing_subscriber::fmt::init();
    Beacon::run(Settings {
        window: window::Settings {
            size: (430, 800),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

enum Beacon {
    Loading,
    Loaded(State),
}

struct State {
    input_value: HashMap<usize, String>,
    eval_cells: HistoryMap,
    tab_at: usize,
    dirty: bool,
    saving: bool,
    panes: pane_grid::State<Pane>,
    panes_created: usize,
    focus: Option<pane_grid::Pane>,
}

impl Default for State {
    fn default() -> Self {
        let (panes, _) = pane_grid::State::new(Pane::new(0));
        Self {
            input_value: HashMap::new(),
            eval_cells: Default::default(),
            tab_at: Default::default(),
            dirty: Default::default(),
            saving: Default::default(),
            panes,
            panes_created: 1,
            focus: None,
        }
    }
}

impl State {
    fn focused_pane(&self) -> usize {
        if let Some(f) = self.focus {
            unsafe { std::mem::transmute::<_, usize>(f) }
        } else {
            0
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<SavedState, LoadError>),
    FontLoaded(Result<(), iced_runtime::font::Error>),
    Saved(Result<(), SaveError>),
    InputChanged(String, usize),
    RunInput,
    InputFocus,
    FillInput(String),
    ToolbarClick(String),
    TabChanged(usize),
    TabCreate,
    TabClose,
    TabNext,
    TabPrev,
    BufferClear,
    Split(pane_grid::Axis, pane_grid::Pane),
    SplitFocused(pane_grid::Axis),
    FocusAdjacent(pane_grid::Direction),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    TogglePin(pane_grid::Pane),
    Maximize(pane_grid::Pane),
    Restore,
    Close(pane_grid::Pane),
    CloseFocused,
}

impl Application for Beacon {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Beacon, Command<Message>) {
        // let _ = REPL.call1(&libs.to_string().into());
        #[cfg(feature = "k")]
        ngnk::kinit();
        (
            Beacon::Loading,
            Command::batch(vec![
                load(include_bytes!("../assets/BQN386.ttf").as_slice()).map(Message::FontLoaded),
                Command::perform(SavedState::load(), Message::Loaded),
            ]),
        )
    }
    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn title(&self) -> String {
        let dirty = match self {
            Beacon::Loading => false,
            Beacon::Loaded(state) => state.dirty,
        };
        let idx = match self {
            Beacon::Loading => 0,
            Beacon::Loaded(state) => state.tab_at,
        };
        format!("Beacon {} - Tab {}", if dirty { "*" } else { "" }, idx)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            Beacon::Loading => {
                match message {
                    Message::Loaded(Ok(state)) => {
                        *self = Beacon::Loaded(State {
                            input_value: state.input_val,
                            eval_cells: {
                                // let mut h = HashMap::new();
                                // h.insert(state.at, state.history);
                                state.history
                            },
                            ..State::default()
                        });
                    }
                    Message::Loaded(Err(_)) => {
                        *self = Beacon::Loaded(State {
                            eval_cells: { HistoryMap::new() },
                            ..State::default()
                        });
                    }
                    _ => {}
                }

                text_input::focus(INPUT_ID.clone())
            }
            Beacon::Loaded(state) => {
                let mut saved = false;

                let max_idx = *state.eval_cells.max_tab();
                let min_idx = *state.eval_cells.min_tab();
                let focused_pane = state.focused_pane();
                let command = match message {
                    Message::Split(axis, pane) => {
                        let result = state
                            .panes
                            .split(axis, &pane, Pane::new(state.panes_created));

                        if let Some((pane, _)) = result {
                            state.focus = Some(pane);
                        }

                        state.panes_created += 1;
                        Command::none()
                    }
                    Message::SplitFocused(axis) => {
                        if let Some(pane) = state.focus {
                            let result =
                                state
                                    .panes
                                    .split(axis, &pane, Pane::new(state.panes_created));

                            if let Some((pane, _)) = result {
                                state.focus = Some(pane);
                            }

                            state.panes_created += 1;
                        }
                        Command::none()
                    }
                    Message::FocusAdjacent(direction) => {
                        if let Some(pane) = state.focus {
                            if let Some(adjacent) = state.panes.adjacent(&pane, direction) {
                                state.focus = Some(adjacent);
                            }
                        }
                        Command::none()
                    }
                    Message::Clicked(pane) => {
                        state.focus = Some(pane);
                        Command::none()
                    }
                    Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                        state.panes.resize(&split, ratio);
                        Command::none()
                    }
                    Message::Dragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                        state.panes.drop(&pane, target);
                        Command::none()
                    }
                    Message::Dragged(_) => Command::none(),
                    Message::TogglePin(pane) => {
                        if let Some(Pane { is_pinned, .. }) = state.panes.get_mut(&pane) {
                            *is_pinned = !*is_pinned;
                        }
                        Command::none()
                    }
                    Message::Maximize(pane) => {
                        state.panes.maximize(&pane);

                        Command::none()
                    }
                    Message::Restore => {
                        state.panes.restore();
                        Command::none()
                    }
                    Message::Close(pane) => {
                        if let Some((_, sibling)) = state.panes.close(&pane) {
                            state.focus = Some(sibling);
                        }
                        Command::none()
                    }
                    Message::CloseFocused => {
                        if let Some(pane) = state.focus {
                            if let Some(Pane { is_pinned, .. }) = state.panes.get(&pane) {
                                if !is_pinned {
                                    if let Some((_, sibling)) = state.panes.close(&pane) {
                                        state.focus = Some(sibling);
                                    }
                                }
                            }
                        }
                        Command::none()
                    }
                    Message::InputChanged(value, pane_idx) => {
                        let i = state.input_value.entry(pane_idx).or_insert(String::new());
                        let ic = state.input_value.get_mut(&focused_pane).unwrap();
                        *ic = value;
                        Command::none()
                    }
                    Message::TabCreate => {
                        let h = HashMap::new();
                        state.eval_cells.0.insert(max_idx + 1, h);
                        state.tab_at = max_idx + 1;
                        Command::none()
                    }
                    Message::TabClose => {
                        if max_idx > 0 {
                            if state.tab_at == min_idx {
                                state.tab_at += 1;
                                state.eval_cells.0.remove(&(state.tab_at - 1));
                            } else {
                                state.tab_at -= 1;
                                state.eval_cells.0.remove(&(state.tab_at + 1));
                            }
                        }
                        Command::none()
                    }
                    Message::TabNext => {
                        if state.tab_at != max_idx {
                            state.tab_at += 1
                        } else {
                            state.tab_at = 0
                        }
                        Command::none()
                    }
                    Message::TabPrev => {
                        if state.tab_at != min_idx {
                            state.tab_at -= 1
                        } else {
                            state.tab_at = max_idx
                        }
                        Command::none()
                    }
                    Message::TabChanged(i) => {
                        state.tab_at = i;
                        Command::none()
                    }
                    Message::FillInput(o) => {
                        let i = state.input_value.get_mut(&focused_pane).unwrap();
                        *i = o;
                        Command::none()
                    }
                    Message::ToolbarClick(c) => {
                        let i = state.input_value.get_mut(&focused_pane).unwrap();
                        *i += c.as_str();
                        Command::none()
                    }
                    Message::Saved(_) => {
                        state.saving = false;
                        saved = true;
                        Command::none()
                    }
                    Message::BufferClear => {
                        if let Some(h) = state.eval_cells.0.get_mut(&state.tab_at) {
                            h.insert(focused_pane, vec![]);
                        }
                        Command::none()
                    }
                    Message::InputFocus =>
                    // FIXME
                    // text_input::focus::<Message>(INPUT_ID.clone()),
                    // Message::InputFocus,
                    {
                        Command::none()
                    }
                    Message::RunInput => {
                        let inp: String = state.input_value[&focused_pane].clone();
                        if inp == "clear" {
                            state.eval_cells = HistoryMap::new();
                            state.tab_at = 0;
                            return Command::none();
                        }
                        if inp == "clean" {
                            // if let Some(a) = state.eval_cells.0.get_mut(&state.tab_at) {
                            //     a.retain(|v| !matches!(v.ty, Ty::Err));
                            // }
                            return Command::none();
                        }
                        if inp == "close" {
                            return Command::perform(async { Message::TabClose }, |_| {
                                Message::TabClose
                            });
                        }
                        let now = Instant::now();
                        let bqnc = REPL.call1(&inp.clone().into());
                        #[cfg(feature = "k")]
                        println!("{}", k::keval("`0:3+3", vec![]));
                        let elapsed = now.elapsed();
                        if let Some(h) = state.eval_cells.0.get_mut(&state.tab_at) {
                            let vec = h.entry(focused_pane).or_insert(vec![]);
                            vec.push(EvalCell {
                                res: truncate(
                                    match bqnc {
                                        Ok(b) => format!("{b:?}"),
                                        Err(e) => format!("{e}"),
                                    }
                                    .as_str(),
                                    500,
                                )
                                .to_string(),
                                src: inp,
                                ty: { Ty::Number },
                                time: elapsed,
                            });
                        }
                        scrollable::snap_to(
                            SCROLL_ID.clone(),
                            scrollable::RelativeOffset { x: 0.0, y: 1.0 },
                        )
                    }
                    _ => Command::none(),
                };

                if !saved {
                    state.dirty = true;
                }

                let save = if state.dirty && !state.saving {
                    state.dirty = false;
                    state.saving = true;

                    Command::perform(
                        SavedState {
                            input_val: state.input_value.clone(),
                            history: state.eval_cells.clone(),
                            at: 0,
                        }
                        .save(),
                        Message::Saved,
                    )
                } else {
                    Command::none()
                };

                Command::batch(vec![command, save])
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            Beacon::Loading => text("loading").into(),
            Beacon::Loaded(
                s @ State {
                    input_value,
                    eval_cells: outs,
                    tab_at: at,
                    focus,
                    panes,
                    ..
                },
            ) => {
                let glyphbar = toolbar_view();
                let tabs = tab_view(outs, *at);

                let focus = focus;
                let total_panes = panes.len();
                let pane_grid = PaneGrid::new(&panes, |id, pane, is_maximized| {
                    let is_focused = *focus == Some(id);
                    let pane_outs = outs
                        .0
                        .get(at)
                        .and_then(|h| h.get(unsafe { &std::mem::transmute::<_, usize>(id) }));
                    pane_grid::Content::new(responsive(move |size| {
                        view_pane(
                            id,
                            total_panes,
                            pane.is_pinned,
                            &input_value,
                            pane_outs,
                            *at,
                        )
                        .into()
                    }))
                })
                .width(Length::Fill)
                .height(Length::Fill)
                .spacing(10)
                .on_click(Message::Clicked)
                .on_drag(Message::Dragged)
                .on_resize(10, Message::Resized);
                container(column![glyphbar, tabs, pane_grid])
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(10)
                    .style(CanvasStyle::theme())
                    .into()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        macro_rules! kp {
            ($kc:pat, $modif:pat) => {
                (
                    Event::Keyboard(keyboard::Event::KeyPressed {
                        key_code: $kc,
                        modifiers: $modif,
                    }),
                    event::Status::Ignored,
                )
            };
        }
        subscription::events_with(|event, status| match (event, status) {
            kp!(keyboard::KeyCode::T, Modifiers::CTRL) => Some(Message::TabCreate),
            kp!(keyboard::KeyCode::Q, Modifiers::CTRL) => Some(Message::TabClose),
            kp!(keyboard::KeyCode::N, Modifiers::CTRL) => Some(Message::TabNext),
            kp!(keyboard::KeyCode::P, Modifiers::CTRL) => Some(Message::TabPrev),
            kp!(keyboard::KeyCode::L, Modifiers::CTRL) => Some(Message::BufferClear),
            (Event::Keyboard(keyboard::Event::CharacterReceived(_)), event::Status::Ignored) => {
                Some(Message::InputFocus)
            }
            _ => None,
        })
    }
}
