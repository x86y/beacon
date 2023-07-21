use cbqn::{eval, BQNValue};
use iced::{
    event::{self, Event},
    keyboard::{self, Modifiers},
    subscription,
    widget::{button, column, container, row, scrollable, text, Column, Container},
    window, Application, Command, Element, Font, Length, Settings, Subscription, Theme,
};
use itertools::Itertools;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use std::{collections::HashMap, time::Duration};

mod save;
mod styles;
mod text_input;
mod wrap;
use crate::save::*;
use crate::styles::*;
use crate::text_input::text_input;
use crate::wrap::Wrap;

static REPL: Lazy<BQNValue> = Lazy::new(|| {
    dbg!(eval(
        "(•ReBQN{repl⇐\"loose\"})⎊{𝕊: \"Error: \"∾•CurrentError@}"
    ))
});
static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);
static SCROLL_ID: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);
static GLYPHS: Lazy<[&str; 64]> = Lazy::new(|| {
    [
        "+", "¨", "⊸", "⊑", "´", "∾", "×", "-", "≠", "∘", "˜", "=", "/", "<", "↕", "⥊", "⊢", "⟜",
        "⊏", "≡", "∧", "˘", "!", ">", "⌽", "↓", "¬", "↑", "∨", "`", "◶", "⍟", "⌜", "⊣", "⌾", "⌈",
        "⋈", "⊔", "⌊", "»", "⊐", "∊", "○", "≤", "|", "≢", "⍉", "÷", "≍", "˝", "⁼", "«", "≥", "˙",
        "⍋", "⍷", "⋆", "⊘", "⎉", "⚇", "⊒", "√", "⍒", "⎊",
    ]
});
macro_rules! bqn386 {
    ($q:expr) => {
        text($q).font(Font::External {
            name: "BQN386",
            bytes: include_bytes!("../assets/BQN386.ttf"),
        })
    };
}
macro_rules! _iosevka {
    ($q:expr) => {
        text($q).font(Font::External {
            name: "Iosevka",
            bytes: include_bytes!("../assets/iosevka-term-medium.ttf"),
        })
    };
}

pub fn main() -> iced::Result {
    Beacon::run(Settings {
        window: window::Settings {
            size: (430, 800),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug)]
enum Beacon {
    Loading,
    Loaded(State),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EvalCell {
    src: String,
    res: String,
    time: Duration,
}

#[derive(Debug, Default)]
struct State {
    input_value: String,
    eval_cells: History,
    tab_at: usize,
    dirty: bool,
    saving: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct History(HashMap<usize, Vec<EvalCell>>);
impl History {
    fn min_tab(&self) -> &usize {
        self.0.keys().min().unwrap_or(&0)
    }
    fn max_tab(&self) -> &usize {
        self.0.keys().max().unwrap_or(&0)
    }
    fn new() -> History {
        let mut h = HashMap::new();
        h.insert(0, vec![]);
        History(h)
    }
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<SavedState, LoadError>),
    Saved(Result<(), SaveError>),
    InputChanged(String),
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
}

impl Application for Beacon {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Beacon, Command<Message>) {
        (
            Beacon::Loading,
            Command::perform(SavedState::load(), Message::Loaded),
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
                            eval_cells: state.history,
                            ..State::default()
                        });
                    }
                    Message::Loaded(Err(_)) => {
                        *self = Beacon::Loaded(State {
                            eval_cells: { History::new() },
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
                let command = match message {
                    Message::InputChanged(value) => {
                        state.input_value = value;
                        Command::none()
                    }
                    Message::TabCreate => {
                        state.eval_cells.0.insert(max_idx + 1, vec![]);
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
                        state.input_value = o;
                        Command::none()
                    }
                    Message::ToolbarClick(c) => {
                        state.input_value += c.as_str();
                        Command::none()
                    }
                    Message::Saved(_) => {
                        state.saving = false;
                        saved = true;
                        Command::none()
                    }
                    Message::BufferClear => {
                        *state.eval_cells.0.get_mut(&state.tab_at).unwrap() = vec![];
                        Command::none()
                    }
                    Message::InputFocus => {
                        text_input::focus::<Message>(INPUT_ID.clone()); //FIXME doesn't focus for
                                                                        //some reason
                        Command::none()
                    }
                    Message::RunInput => {
                        if state.input_value == "clear" {
                            state.eval_cells = History::new();
                            state.tab_at = 0;
                            return Command::none();
                        }
                        if state.input_value == "close" {
                            return Command::perform(async { Message::TabClose }, |_| {
                                Message::TabClose
                            });
                        }
                        let now = Instant::now();
                        let bqnv = REPL.call1(&state.input_value.clone().into());
                        let elapsed = now.elapsed();
                        fn truncate(s: &str, max_chars: usize) -> &str {
                            match s.char_indices().nth(max_chars) {
                                None => s,
                                Some((idx, _)) => &s[..idx],
                            }
                        }
                        state
                            .eval_cells
                            .0
                            .get_mut(&state.tab_at)
                            .unwrap()
                            .push(EvalCell {
                                res: truncate(format!("{bqnv:?}").as_str(), 500).to_string(),
                                src: state.input_value.clone(),
                                time: elapsed,
                            });
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
            Beacon::Loading => bqn386!("loading").into(),
            Beacon::Loaded(State {
                input_value,
                eval_cells: outs,
                tab_at: at,
                ..
            }) => {
                let inp = text_input("", input_value, Message::InputChanged)
                    .padding(15)
                    .style(InputStyle::theme())
                    .size(24)
                    .font(Font::External {
                        name: "BQN386",
                        bytes: include_bytes!("../assets/BQN386.ttf"),
                    })
                    .on_submit(Message::RunInput)
                    .id(INPUT_ID.clone());
                let glyphbar: Container<_> = Container::new(
                    GLYPHS
                        .iter()
                        .fold(Wrap::new(), |wrap, glyph| {
                            wrap.push(
                                button(bqn386!(glyph).size(24))
                                    .style(BtnStyle::theme())
                                    .on_press(Message::ToolbarClick(glyph.to_string())),
                            )
                        })
                        .spacing(1),
                );
                let out_cells: Column<_> = column(
                    outs.0[at]
                        .iter()
                        .map(|txt| {
                            let mut res = txt.res.to_string();
                            let mut did_error = false;
                            if res.contains("Error") {
                                res = res.replace('\"', "");
                                did_error = true;
                            }
                            let mut v = vec![
                                button(
                                    bqn386!(" ".to_string() + &txt.src)
                                        .size(20)
                                        .style(SrcCellStyle::theme()),
                                )
                                .on_press(Message::FillInput(txt.src.to_string()))
                                .style(BtnStyle::theme())
                                .into(),
                                button(
                                    bqn386!(res.clone())
                                        .style(if did_error {
                                            ErroredCellStyle::theme()
                                        } else {
                                            Default::default()
                                        })
                                        .size(20),
                                )
                                .on_press(Message::FillInput(res))
                                .style(BtnStyle::theme())
                                .into(),
                            ];
                            if !did_error {
                                v.push(
                                    bqn386!(format!("{}ms", txt.time.as_millis()))
                                        .style(ElapsedTimeStyle::theme())
                                        .size(16)
                                        .into(),
                                )
                            }
                            Container::new(column(v))
                                .width(Length::Fill)
                                .style(CanvasStyle::theme())
                                .into()
                        })
                        .collect::<Vec<Element<_>>>(),
                )
                .spacing(8);
                let tabs: Column<Message> = column({
                    outs.0
                        .keys()
                        .sorted()
                        .map(|i| {
                            Container::new(
                                button(if i == at {
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
                let content = row![
                    column![tabs, new_tab_btn],
                    column![
                        container(glyphbar).style(ToolbarStyle::theme()),
                        scrollable(out_cells)
                            .height(Length::Fill)
                            .id(SCROLL_ID.clone()),
                        inp
                    ]
                    .spacing(20)
                    .max_width(800)
                ];
                container(content)
                    .width(Length::Fill)
                    .center_x()
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
