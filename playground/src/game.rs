use std::collections::HashMap;
use shipyard::*;
use tetra::graphics::DrawParams;
use tetra::graphics::{self, Color, Texture};
use tetra::math::Vec2;
use tetra::{ input, Context, ContextBuilder, Result, State, Trans };
use tetra::input::*;

//
// This is just temporary because I'm not sure how we want to handle loading resources to each state
// So just making it work without resources for now
struct Resources;
impl Resources {
    fn none(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Resources)
    }
}

// This is just a wrapper for creating a Tetra Context
// I didn't look through the whole Tetra config so most of the options aren't exposed
pub struct Game {
    pub title: String,
    pub window_width: i32,
    pub window_height: i32
}

impl Game {
    pub fn new<S>(title: S, window_width: i32, window_height: i32) -> Self 
        where S: std::string::ToString {
        Game{ title: title.to_string(), window_width, window_height }
    }

    pub fn launch(&self, state: GameState) {
        if let Err(err) = ContextBuilder::new(&self.title, self.window_width, self.window_height)
            .show_mouse(true)
            .build()
            .unwrap()
            .run(|_| Ok(state), Resources::none) 
            {
                panic!(err)
            }
    }
}

use InputAction::*;
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum InputAction {
    Pressed(Input),
    Released(Input),
    Held(Input)
}

// Yikes
use Input::*;
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Input {
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    KeyNum0,
    KeyNum1,
    KeyNum2,
    KeyNum3,
    KeyNum4,
    KeyNum5,
    KeyNum6,
    KeyNum7,
    KeyNum8,
    KeyNum9,
    KeyF1,
    KeyF2,
    KeyF3,
    KeyF4,
    KeyF5,
    KeyF6,
    KeyF7,
    KeyF8,
    KeyF9,
    KeyF10,
    KeyF11,
    KeyF12,
    KeyF13,
    KeyF14,
    KeyF15,
    KeyF16,
    KeyF17,
    KeyF18,
    KeyF19,
    KeyF20,
    KeyF21,
    KeyF22,
    KeyF23,
    KeyF24,
    KeyNumLock,
    KeyNumPad1,
    KeyNumPad2,
    KeyNumPad3,
    KeyNumPad4,
    KeyNumPad5,
    KeyNumPad6,
    KeyNumPad7,
    KeyNumPad8,
    KeyNumPad9,
    KeyNumPad0,
    KeyNumPadPlus,
    KeyNumPadMinus,
    KeyNumPadMultiply,
    KeyNumPadDivide,
    KeyNumPadEnter,
    KeyLeftCtrl,
    KeyLeftShift,
    KeyLeftAlt,
    KeyRightCtrl,
    KeyRightShift,
    KeyRightAlt,
    KeyUp,
    KeyDown,
    KeyLeft,
    KeyRight,
    KeyAmpersand,
    KeyAsterisk,
    KeyAt,
    KeyBackquote,
    KeyBackslash,
    KeyBackspace,
    KeyCapsLock,
    KeyCaret,
    KeyColon,
    KeyComma,
    KeyDelete,
    KeyDollar,
    KeyDoubleQuote,
    KeyEnd,
    KeyEnter,
    KeyEquals,
    KeyEscape,
    KeyExclaim,
    KeyGreaterThan,
    KeyHash,
    KeyHome,
    KeyInsert,
    KeyLeftBracket,
    KeyLeftParen,
    KeyLessThan,
    KeyMinus,
    KeyPageDown,
    KeyPageUp,
    KeyPause,
    KeyPercent,
    KeyPeriod,
    KeyPlus,
    KeyPrintScreen,
    KeyQuestion,
    KeyQuote,
    KeyRightBracket,
    KeyRightParen,
    KeyScrollLock,
    KeySemicolon,
    KeySlash,
    KeySpace,
    KeyTab,
    KeyUnderscore,
    MouseLeft,
    MouseRight,
    MouseMiddle,
    MouseX1,
    MouseX2,
}

// Also yikes
impl Input {
    fn from_tetra_key(key :Key) -> Input {
        match key {
            Key::A => KeyA,
            Key::B => KeyB,
            Key::C => KeyC,
            Key::D => KeyD,
            Key::E => KeyE,
            Key::F => KeyF,
            Key::G => KeyG,
            Key::H => KeyH,
            Key::I => KeyI,
            Key::J => KeyJ,
            Key::K => KeyK,
            Key::L => KeyL,
            Key::M => KeyM,
            Key::N => KeyN,
            Key::O => KeyO,
            Key::P => KeyP,
            Key::Q => KeyQ,
            Key::R => KeyR,
            Key::S => KeyS,
            Key::T => KeyT,
            Key::U => KeyU,
            Key::V => KeyV,
            Key::W => KeyW,
            Key::X => KeyX,
            Key::Y => KeyY,
            Key::Z => KeyZ,
            Key::Num0 => KeyNum0,
            Key::Num1 => KeyNum1,
            Key::Num2 => KeyNum2,
            Key::Num3 => KeyNum3,
            Key::Num4 => KeyNum4,
            Key::Num5 => KeyNum5,
            Key::Num6 => KeyNum6,
            Key::Num7 => KeyNum7,
            Key::Num8 => KeyNum8,
            Key::Num9 => KeyNum9,
            Key::F1 => KeyF1,
            Key::F2 => KeyF2,
            Key::F3 => KeyF3,
            Key::F4 => KeyF4,
            Key::F5 => KeyF5,
            Key::F6 => KeyF6,
            Key::F7 => KeyF7,
            Key::F8 => KeyF8,
            Key::F9 => KeyF9,
            Key::F10 => KeyF10,
            Key::F11 => KeyF11,
            Key::F12 => KeyF12,
            Key::F13 => KeyF13,
            Key::F14 => KeyF14,
            Key::F15 => KeyF15,
            Key::F16 => KeyF16,
            Key::F17 => KeyF17,
            Key::F18 => KeyF18,
            Key::F19 => KeyF19,
            Key::F20 => KeyF20,
            Key::F21 => KeyF21,
            Key::F22 => KeyF22,
            Key::F23 => KeyF23,
            Key::F24 => KeyF24,
            Key::NumLock => KeyNumLock,
            Key::NumPad1 => KeyNumPad1,
            Key::NumPad2 => KeyNumPad2,
            Key::NumPad3 => KeyNumPad3,
            Key::NumPad4 => KeyNumPad4,
            Key::NumPad5 => KeyNumPad5,
            Key::NumPad6 => KeyNumPad6,
            Key::NumPad7 => KeyNumPad7,
            Key::NumPad8 => KeyNumPad8,
            Key::NumPad9 => KeyNumPad9,
            Key::NumPad0 => KeyNumPad0,
            Key::NumPadPlus => KeyNumPadPlus,
            Key::NumPadMinus => KeyNumPadMinus,
            Key::NumPadMultiply => KeyNumPadMultiply,
            Key::NumPadDivide => KeyNumPadDivide,
            Key::NumPadEnter => KeyNumPadEnter,
            Key::LeftCtrl => KeyLeftCtrl,
            Key::LeftShift => KeyLeftShift,
            Key::LeftAlt => KeyLeftAlt,
            Key::RightCtrl => KeyRightCtrl,
            Key::RightShift => KeyRightShift,
            Key::RightAlt => KeyRightAlt,
            Key::Up => KeyUp,
            Key::Down => KeyDown,
            Key::Left => KeyLeft,
            Key::Right => KeyRight,
            Key::Ampersand => KeyAmpersand,
            Key::Asterisk => KeyAsterisk,
            Key::At => KeyAt,
            Key::Backquote => KeyBackquote,
            Key::Backslash => KeyBackslash,
            Key::Backspace => KeyBackspace,
            Key::CapsLock => KeyCapsLock,
            Key::Caret => KeyCaret,
            Key::Colon => KeyColon,
            Key::Comma => KeyComma,
            Key::Delete => KeyDelete,
            Key::Dollar => KeyDollar,
            Key::DoubleQuote => KeyDoubleQuote,
            Key::End => KeyEnd,
            Key::Enter => KeyEnter,
            Key::Equals => KeyEquals,
            Key::Escape => KeyEscape,
            Key::Exclaim => KeyExclaim,
            Key::GreaterThan => KeyGreaterThan,
            Key::Hash => KeyHash,
            Key::Home => KeyHome,
            Key::Insert => KeyInsert,
            Key::LeftBracket => KeyLeftBracket,
            Key::LeftParen => KeyLeftParen,
            Key::LessThan => KeyLessThan,
            Key::Minus => KeyMinus,
            Key::PageDown => KeyPageDown,
            Key::PageUp => KeyPageUp,
            Key::Pause => KeyPause,
            Key::Percent => KeyPercent,
            Key::Period => KeyPeriod,
            Key::Plus => KeyPlus,
            Key::PrintScreen => KeyPrintScreen,
            Key::Question => KeyQuestion,
            Key::Quote => KeyQuote,
            Key::RightBracket => KeyRightBracket,
            Key::RightParen => KeyRightParen,
            Key::ScrollLock => KeyScrollLock,
            Key::Semicolon => KeySemicolon,
            Key::Slash => KeySlash,
            Key::Space => KeySpace,
            Key::Tab => KeyTab,
            Key::Underscore => KeyUnderscore,
        }
    }
}

pub type Controls = HashMap<InputAction, &'static str>;

pub struct GameState {
    workload: String,
    world: World,
    controls: Controls
}

impl State<Resources> for GameState {
    fn update(&mut self, ctx: &mut Context, _: &mut Resources) -> Result<Trans<Resources>> {
        self.handle_input(ctx);
        self.world.run_workload(&self.workload);

        // Right now there's no transitions since there's no way to access this outside the game state
        // I want to add a transition handler that we can pull out of the shipyard world
        Ok(Trans::None)
    }

    fn draw(&mut self, ctx: &mut Context, _resources: &mut Resources) -> tetra::Result {
        // Cornflower blue, as is tradition
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        // Uhhhh this example doesn't render anything
        // We caaaaan work out a simple system to draw everything here using Renderable components
        // I think it good to start with that and figure out something more advanced later

        Ok(())
    }
}

impl GameState {
    pub fn new<S: std::string::ToString>(workload: S, world: World, controls: Controls) -> GameState {
        GameState { workload: workload.to_string(), world, controls }
    }

    // Yikes
    // Later on I wanna transform this into an input Context and throw it into the world for system access
    // For now though this works to map workloads onto key actions
    fn handle_input(&mut self, ctx: &Context) {
        for key in input::get_keys_pressed(ctx) {
            if self.controls.contains_key(&Pressed(Input::from_tetra_key(*key))) {
                self.world.run_workload(&self.controls[&Pressed(Input::from_tetra_key(*key))]);
            }
        }
        for key in input::get_keys_down(ctx) {
            if self.controls.contains_key(&Held(Input::from_tetra_key(*key))) {
                self.world.run_workload(&self.controls[&Held(Input::from_tetra_key(*key))]);
            }
        }
        for key in input::get_keys_released(ctx) {
            if self.controls.contains_key(&Released(Input::from_tetra_key(*key))) {
                self.world.run_workload(&self.controls[&Released(Input::from_tetra_key(*key))]);
            }
        }
        if input::is_mouse_button_pressed(ctx, MouseButton::Left) {
            if self.controls.contains_key(&Pressed(MouseLeft)) {
                self.world.run_workload(&self.controls[&Pressed(MouseLeft)]);
            }
        }
        if input::is_mouse_button_pressed(ctx, MouseButton::Middle) {
            if self.controls.contains_key(&Pressed(MouseMiddle)) {
                self.world.run_workload(&self.controls[&Pressed(MouseMiddle)]);
            }
        }
        if input::is_mouse_button_pressed(ctx, MouseButton::Right) {
            if self.controls.contains_key(&Pressed(MouseRight)) {
                self.world.run_workload(&self.controls[&Pressed(MouseRight)]);
            }
        }
        if input::is_mouse_button_pressed(ctx, MouseButton::X1) {
            if self.controls.contains_key(&Pressed(MouseX1)) {
                self.world.run_workload(&self.controls[&Pressed(MouseX1)]);
            }
        }
        if input::is_mouse_button_pressed(ctx, MouseButton::X2) {
            if self.controls.contains_key(&Pressed(MouseX2)) {
                self.world.run_workload(&self.controls[&Pressed(MouseX2)]);
            }
        }
        if input::is_mouse_button_down(ctx, MouseButton::Left) {
            if self.controls.contains_key(&Held(MouseLeft)) {
                self.world.run_workload(&self.controls[&Held(MouseLeft)]);
            }
        }
        if input::is_mouse_button_down(ctx, MouseButton::Middle) {
            if self.controls.contains_key(&Held(MouseMiddle)) {
                self.world.run_workload(&self.controls[&Held(MouseMiddle)]);
            }
        }
        if input::is_mouse_button_down(ctx, MouseButton::Right) {
            if self.controls.contains_key(&Held(MouseRight)) {
                self.world.run_workload(&self.controls[&Held(MouseRight)]);
            }
        }
        if input::is_mouse_button_down(ctx, MouseButton::X1) {
            if self.controls.contains_key(&Held(MouseX1)) {
                self.world.run_workload(&self.controls[&Held(MouseX1)]);
            }
        }
        if input::is_mouse_button_down(ctx, MouseButton::X2) {
            if self.controls.contains_key(&Held(MouseX2)) {
                self.world.run_workload(&self.controls[&Held(MouseX2)]);
            }
        }
        if input::is_mouse_button_released(ctx, MouseButton::Left) {
            if self.controls.contains_key(&Released(MouseLeft)) {
                self.world.run_workload(&self.controls[&Released(MouseLeft)]);
            }
        }
        if input::is_mouse_button_released(ctx, MouseButton::Middle) {
            if self.controls.contains_key(&Released(MouseMiddle)) {
                self.world.run_workload(&self.controls[&Released(MouseMiddle)]);
            }
        }
        if input::is_mouse_button_released(ctx, MouseButton::Right) {
            if self.controls.contains_key(&Released(MouseRight)) {
                self.world.run_workload(&self.controls[&Released(MouseRight)]);
            }
        }
        if input::is_mouse_button_released(ctx, MouseButton::X1) {
            if self.controls.contains_key(&Released(MouseX1)) {
                self.world.run_workload(&self.controls[&Released(MouseX1)]);
            }
        }
        if input::is_mouse_button_released(ctx, MouseButton::X2) {
            if self.controls.contains_key(&Released(MouseX2)) {
                self.world.run_workload(&self.controls[&Released(MouseX2)]);
            }
        }
    }
}