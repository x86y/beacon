use cbqn::{eval, BQNValue};
use iced::{
    event::{self, Event},
    keyboard::{self, Modifiers},
    subscription,
    widget::{button, column, container, row, scrollable, text, tooltip, Column, Container},
    window, Application, Command, Element, Length, Settings, Subscription, Theme,
};
use iced_core::{text::LineHeight, Font};
use iced_runtime::font::load;
use itertools::Itertools;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use std::{collections::HashMap, time::Duration};
// use tracing::{event as e, info, instrument, Level};

mod docs;
#[cfg(feature = "k")]
mod k;
mod save;
mod styles;
mod widgets;
use crate::save::*;
use crate::styles::*;
use crate::widgets::text_input;
use crate::widgets::wrap::Wrap;
use docs::content::glyph_to_documentation;

static REPL: Lazy<BQNValue> = Lazy::new(|| {
    eval("(•ReBQN{repl⇐\"loose\"})⎊{𝕊: \"Error: \"∾•CurrentError@}")
        .expect("Err on repl construction")
});
static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);
static SCROLL_ID: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);
static GLYPHS: Lazy<[char; 64]> = Lazy::new(|| {
    [
        '+', '¨', '⊸', '⊑', '´', '∾', '×', '-', '≠', '∘', '˜', '=', '/', '<', '↕', '⥊', '⊢', '⟜',
        '⊏', '≡', '∧', '˘', '!', '>', '⌽', '↓', '¬', '↑', '∨', '`', '◶', '⍟', '⌜', '⊣', '⌾', '⌈',
        '⋈', '⊔', '⌊', '»', '⊐', '∊', '○', '≤', '|', '≢', '⍉', '÷', '≍', '˝', '⁼', '«', '≥', '˙',
        '⍋', '⍷', '⋆', '⊘', '⎉', '⚇', '⊒', '√', '⍒', '⎊',
    ]
});
macro_rules! bqn386 {
    ($q:expr) => {
        text($q)
            .font(Font::with_name("BQN386 Unicode"))
            .size(14)
            .line_height(LineHeight::Absolute(12.into()))
    };
}

pub fn main() -> iced::Result {
    tracing_subscriber::fmt::init();
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
enum Ty {
    Array,
    Number,
    Character,
    Function,
    Mod1,
    Mod2,
    Namespace,
    Err,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EvalCell {
    src: String,
    res: String,
    ty: Ty,
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
    FontLoaded(Result<(), iced_runtime::font::Error>),
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
                    Message::InputFocus =>
                    // FIXME
                    // text_input::focus::<Message>(INPUT_ID.clone()),
                    // Message::InputFocus,
                    {
                        Command::none()
                    }
                    Message::RunInput => {
                        if state.input_value == "clear" {
                            state.eval_cells = History::new();
                            state.tab_at = 0;
                            return Command::none();
                        }
                        if state.input_value == "clean" {
                            if let Some(a) = state.eval_cells.0.get_mut(&state.tab_at) {
                                a.retain(|v| !matches!(v.ty, Ty::Err));
                            }
                            return Command::none();
                        }
                        if state.input_value == "close" {
                            return Command::perform(async { Message::TabClose }, |_| {
                                Message::TabClose
                            });
                        }
                        let now = Instant::now();
                        let bqnc = REPL.call1(&state.input_value.clone().into());
                        #[cfg(feature = "k")]
                        println!("{}", k::keval("`0:3+3", vec![]));
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
                                res: truncate(
                                    match bqnc {
                                        Ok(b) => format!("{b:?}"),
                                        Err(e) => format!("{e}"),
                                    }
                                    .as_str(),
                                    500,
                                )
                                .to_string(),
                                src: { state.input_value.clone() },
                                ty: { Ty::Number },
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
                let inp = text_input::text_input("", input_value)
                    .padding(15)
                    .style(InputStyle::theme())
                    .size(18)
                    .font(Font::with_name("BQN386 Unicode"))
                    .on_submit(Message::RunInput)
                    .on_input(Message::InputChanged)
                    .id(INPUT_ID.clone());
                let glyphbar: Container<_> = Container::new(
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
                                .style(TooltipStyle::theme()),
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
                            if txt.res.starts_with("CBQN error:") {
                                res = res.replace("CBQN error: ", "");
                                did_error = true;
                            }
                            let mut v = vec![
                                button(
                                    bqn386!(" ".to_string() + &txt.src)
                                        .style(SrcCellStyle::theme()),
                                )
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
                let tabs: iced::widget::Row<Message> = row({
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
                let content = row![column![
                    column![
                        container(glyphbar).style(ToolbarStyle::theme()),
                        row![tabs, new_tab_btn],
                    ]
                    .spacing(0),
                    scrollable(out_cells)
                        .height(Length::Fill)
                        .id(SCROLL_ID.clone()),
                    inp
                ]
                .spacing(20)
                .max_width(800),];
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
