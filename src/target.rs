use egui::{ Context, Key, Modifiers, PointerButton };
use std::mem::zeroed;
use windows::Win32::UI::Input::KeyboardAndMouse::GetKeyState;

use crate::keycodes;

use lazy_static::lazy_static;
use std::collections::HashMap;
type MyHashMap = HashMap<i32, bool>;
use std::sync::Mutex;

lazy_static! {
    static ref KEY_HASHMAP: Mutex<MyHashMap> = Mutex::new(MyHashMap::new());
}

fn get_pointer_keycode(button: PointerButton) -> i32 {
    let cool = match button {
        egui::PointerButton::Middle => keycodes::VK_MBUTTON,
        PointerButton::Primary => keycodes::VK_LBUTTON,
        PointerButton::Secondary => keycodes::VK_RBUTTON,
        PointerButton::Extra1 => keycodes::VK_XBUTTON1,
        PointerButton::Extra2 => keycodes::VK_XBUTTON2,
    };

    cool
}

fn get_keyboard_keycode(key: Key) -> i32 {
    match key {
        Key::ArrowDown => keycodes::VK_DOWN,
        Key::ArrowLeft => keycodes::VK_LEFT,
        Key::ArrowRight => keycodes::VK_RIGHT,
        Key::ArrowUp => keycodes::VK_UP,
        Key::Escape => keycodes::VK_ESCAPE,
        Key::Tab => keycodes::VK_TAB,
        Key::Backspace => keycodes::VK_BACK,
        Key::Enter => keycodes::VK_RETURN,
        Key::Space => keycodes::VK_SPACE,
        Key::Insert => keycodes::VK_INSERT,
        Key::Delete => keycodes::VK_DELETE,
        Key::Home => keycodes::VK_HOME,
        Key::End => keycodes::VK_END,
        Key::PageUp => keycodes::VK_PRIOR,
        Key::PageDown => keycodes::VK_NEXT,
        Key::Cut => keycodes::VK_DELETE,
        Key::Paste => keycodes::VK_INSERT,
        Key::Colon => keycodes::VK_OEM_1,
        Key::Comma => keycodes::VK_OEM_COMMA,
        Key::Backslash => keycodes::VK_OEM_5,
        Key::OpenBracket => keycodes::VK_OEM_4,
        Key::CloseBracket => keycodes::VK_OEM_6,
        Key::Backtick => keycodes::VK_OEM_3,
        Key::Minus => keycodes::VK_OEM_MINUS,
        Key::Period => keycodes::VK_OEM_PERIOD,
        Key::Plus => keycodes::VK_OEM_PLUS,
        Key::Equals => keycodes::VK_OEM_PLUS,
        Key::Semicolon => keycodes::VK_OEM_1,
        Key::Num0 => keycodes::VK_NUMPAD0,
        Key::Num1 => keycodes::VK_NUMPAD1,
        Key::Num2 => keycodes::VK_NUMPAD2,
        Key::Num3 => keycodes::VK_NUMPAD3,
        Key::Num4 => keycodes::VK_NUMPAD4,
        Key::Num5 => keycodes::VK_NUMPAD5,
        Key::Num6 => keycodes::VK_NUMPAD6,
        Key::Num7 => keycodes::VK_NUMPAD7,
        Key::Num8 => keycodes::VK_NUMPAD8,
        Key::Num9 => keycodes::VK_NUMPAD9,
        Key::A => keycodes::VK_A,
        Key::B => keycodes::VK_B,
        Key::C => keycodes::VK_C,
        Key::D => keycodes::VK_D,
        Key::E => keycodes::VK_E,
        Key::F => keycodes::VK_F,
        Key::G => keycodes::VK_G,
        Key::H => keycodes::VK_H,
        Key::I => keycodes::VK_I,
        Key::J => keycodes::VK_J,
        Key::K => keycodes::VK_K,
        Key::L => keycodes::VK_L,
        Key::M => keycodes::VK_M,
        Key::N => keycodes::VK_N,
        Key::O => keycodes::VK_O,
        Key::P => keycodes::VK_P,
        Key::Q => keycodes::VK_Q,
        Key::R => keycodes::VK_R,
        Key::S => keycodes::VK_S,
        Key::T => keycodes::VK_T,
        Key::U => keycodes::VK_U,
        Key::V => keycodes::VK_V,
        Key::W => keycodes::VK_W,
        Key::X => keycodes::VK_X,
        Key::Y => keycodes::VK_Y,
        Key::Z => keycodes::VK_Z,
        Key::F1 => keycodes::VK_F1,
        Key::F2 => keycodes::VK_F2,
        Key::F3 => keycodes::VK_F3,
        Key::F4 => keycodes::VK_F4,
        Key::F5 => keycodes::VK_F5,
        Key::F6 => keycodes::VK_F6,
        Key::F7 => keycodes::VK_F7,
        Key::F8 => keycodes::VK_F8,
        Key::F9 => keycodes::VK_F9,
        Key::F10 => keycodes::VK_F10,
        Key::F11 => keycodes::VK_F11,
        Key::F12 => keycodes::VK_F12,
        Key::F13 => keycodes::VK_F13,
        Key::F14 => keycodes::VK_F14,
        Key::F15 => keycodes::VK_F15,
        Key::F16 => keycodes::VK_F16,
        Key::F17 => keycodes::VK_F17,
        Key::F18 => keycodes::VK_F18,
        Key::F19 => keycodes::VK_F19,
        Key::F20 => keycodes::VK_F20,
        _ => 0,
    }
}

/// Type that can be used as a bind target
pub trait BindTarget: Clone {
    /// Can accept key bind?
    const IS_KEY: bool;
    /// Can accept pointer bind?
    const IS_POINTER: bool;

    /// Can be cleared?
    const CLEARABLE: bool;

    /// Reported key_pressed
    const REPORTED: bool;

    /// Sets new key bind
    fn set_key(&mut self, key: Key, modifiers: Modifiers);

    /// Sets new pointer bind
    fn set_pointer(&mut self, button: PointerButton, modifiers: Modifiers);

    /// Clears the bind
    fn clear(&mut self);

    /// Formats a bind to a string
    fn format(&self) -> String;

    /// Is bind down?
    fn down(&self) -> bool;

    /// Was bind pressed this frame?
    fn pressed(&self) -> bool;

    /// Was bind released this frame?
    fn released(&self) -> bool;
}

impl BindTarget for Key {
    const IS_KEY: bool = true;
    const IS_POINTER: bool = false;
    const CLEARABLE: bool = false;
    const REPORTED: bool = false;

    fn set_key(&mut self, key: Key, _: Modifiers) {
        *self = key;
    }

    fn set_pointer(&mut self, _: PointerButton, _: Modifiers) {
        unimplemented!()
    }

    fn format(&self) -> String {
        match self {
            Self::Backspace => "BKSP".into(),
            Self::Escape => "ESC".into(),
            Self::Enter => "RET".into(),
            Self::Insert => "INS".into(),
            Self::Delete => "DEL".into(),
            Self::PageUp => "PGU".into(),
            Self::PageDown => "PGD".into(),
            Self::Equals => "=".into(),
            Self::Period => ".".into(),
            Self::Comma => ",".into(),
            Self::Plus => "+".into(),
            Self::Backtick => "`".into(),
            Self::Minus => "-".into(),
            Self::Backslash => "\\".into(),
            Self::Colon => ":".into(),
            Self::Semicolon => ";".into(),
            Self::OpenBracket => "[".into(),
            Self::CloseBracket => "]".into(),
            Self::Num0 => "0".into(),
            Self::Num1 => "1".into(),
            Self::Num2 => "2".into(),
            Self::Num3 => "3".into(),
            Self::Num4 => "4".into(),
            Self::Num5 => "5".into(),
            Self::Num6 => "6".into(),
            Self::Num7 => "7".into(),
            Self::Num8 => "8".into(),
            Self::Num9 => "9".into(),
            _ => format!("{self:?}"),
        }
    }

    fn clear(&mut self) {
        unimplemented!()
    }

    fn down(&self) -> bool {
        let key = unsafe { GetKeyState(get_keyboard_keycode(*self)) };
        let is_down = ((key >> 15) & 1) != 0;

        is_down
    }

    fn pressed(&self) -> bool {
        let is_down = self.down();
        let code = get_keyboard_keycode(*self);


        if is_down && !(KEY_HASHMAP.lock().unwrap().get(&code).unwrap_or(&false).clone()) {
            KEY_HASHMAP.lock().unwrap().remove(&code);
            KEY_HASHMAP.lock().unwrap().insert(code, true);
            return true;
        }

        false
    }

    fn released(&self) -> bool {
        let is_down = self.down();
        let code = get_keyboard_keycode(*self);
        if !is_down && (KEY_HASHMAP.lock().unwrap().get(&code).unwrap_or(&false).clone()) {
            KEY_HASHMAP.lock().unwrap().remove(&code);
            return true;
        }

        false
    }
}

macro_rules! option_through {
    (
        $check:expr,
        $ctx:expr,
        $($path:tt)*
    ) => {
        if let Some(v) = $check {
            v.$($path)*($ctx)
        } else {
            false
        }
    };
}

impl BindTarget for PointerButton {
    const IS_KEY: bool = false;
    const IS_POINTER: bool = true;
    const CLEARABLE: bool = false;
    const REPORTED: bool = false;

    fn set_key(&mut self, _: Key, _: Modifiers) {
        unimplemented!()
    }

    fn set_pointer(&mut self, button: PointerButton, _: Modifiers) {
        let code = get_pointer_keycode(*self);
        KEY_HASHMAP.lock().unwrap().remove(&code);


        *self = button;
    }

    fn clear(&mut self) {
        unimplemented!()
    }

    fn format(&self) -> String {
        (
            match self {
                PointerButton::Extra2 => "M5",
                PointerButton::Extra1 => "M4",
                PointerButton::Middle => "M3",
                PointerButton::Secondary => "M2",
                PointerButton::Primary => "M1",
            }
        ).into()
    }

    fn down(&self) -> bool {
        let key = unsafe { GetKeyState(get_pointer_keycode(*self)) };
        let is_down = ((key >> 15) & 1) != 0;

        is_down
    }

    fn pressed(&self) -> bool {
        let is_down = self.down();
        let code = get_pointer_keycode(*self);

        if is_down && !(KEY_HASHMAP.lock().unwrap().get(&code).unwrap_or(&false).clone()) {
            KEY_HASHMAP.lock().unwrap().remove(&code);
            KEY_HASHMAP.lock().unwrap().insert(code, true);
            return true;
        }

        false
    }

    fn released(&self) -> bool {
        let is_down = self.down();
        let code = get_pointer_keycode(*self);

        if !is_down && (KEY_HASHMAP.lock().unwrap().get(&code).unwrap_or(&false).clone()) {
            KEY_HASHMAP.lock().unwrap().remove(&code);
            return true;
        }

        false
    }
}

impl<B: BindTarget> BindTarget for (B, Modifiers) {
    const IS_KEY: bool = B::IS_KEY;
    const IS_POINTER: bool = B::IS_POINTER;
    const CLEARABLE: bool = false;
    const REPORTED: bool = false;

    fn set_key(&mut self, key: Key, modifiers: Modifiers) {
        self.0.set_key(key, modifiers);
        self.1 = modifiers;
    }

    fn set_pointer(&mut self, button: PointerButton, modifiers: Modifiers) {
        self.0.set_pointer(button, modifiers);
        self.1 = modifiers;
    }

    fn clear(&mut self) {
        unimplemented!();
    }

    fn format(&self) -> String {
        let mut prefix = String::with_capacity(4);
        if self.1.ctrl || self.1.command {
            prefix.push('^');
        }

        if self.1.shift {
            prefix.push('_');
        }

        if self.1.alt {
            prefix.push('*');
        }

        prefix + &self.0.format()
    }

    fn down(&self) -> bool {
        self.0.down()
    }

    fn pressed(&self) -> bool {
        self.0.pressed()
    }

    fn released(&self) -> bool {
        self.0.released()
    }
}

impl<B: BindTarget> BindTarget for Option<(B, Modifiers)> {
    const IS_KEY: bool = B::IS_KEY;
    const IS_POINTER: bool = B::IS_POINTER;
    const CLEARABLE: bool = true;
    const REPORTED: bool = false;

    fn set_key(&mut self, key: Key, modifiers: Modifiers) {
        unsafe {
            (self as *mut Self).write(Some(zeroed()));
        }

        if let Some((b, m)) = self {
            b.set_key(key, modifiers);
            *m = modifiers;
        }
    }

    fn set_pointer(&mut self, button: PointerButton, modifiers: Modifiers) {
        unsafe {
            (self as *mut Self).write(Some(zeroed()));
        }

        if let Some((b, m)) = self {
            b.set_pointer(button, modifiers);
            *m = modifiers;
        }
    }

    fn clear(&mut self) {
        *self = None;
    }

    fn format(&self) -> String {
        self.as_ref()
            .map(BindTarget::format)
            .unwrap_or_else(|| "None".into())
    }

    fn down(&self) -> bool {
        if let Some(v) = self { v.down() } else { false }
    }

    fn pressed(&self) -> bool {
        if let Some(v) = self { v.pressed() } else { false }
    }

    fn released(&self) -> bool {
        if let Some(v) = self { v.released() } else { false }
    }
}

#[test]
fn test_set_opt() {
    let mut b: Option<(Key, Modifiers)> = None;
    let mods = Modifiers {
        alt: true,
        shift: true,
        ctrl: false,
        command: false,
        mac_cmd: false,
    };
    b.set_key(Key::Tab, mods);

    assert_eq!(b, Some((Key::Tab, mods)));
}
