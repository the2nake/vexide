//! Register LCD button callbacks.
//!
//! Use of button callbacks isn't recommended,
//! instead run code when a button is pressed by checking the state of the buttons in a loop.
//! Buttons can be checked with [`read_buttons`].

extern crate alloc;

use alloc::boxed::Box;
use core::fmt::Debug;

use crate::sync::Mutex;

#[derive(Debug)]
/// A snapshot of the state of the buttons on the llemu
pub struct ButtonsState {
    /// Left button pressed state
    pub left_pressed: bool,
    /// Middle button pressed state
    pub middle_pressed: bool,
    /// Right button pressed state
    pub right_pressed: bool,
}

/// Reads the current state of the llemu buttons
pub fn read_buttons() -> ButtonsState {
    let bit_mask = unsafe { pros_sys::lcd_read_buttons() };
    ButtonsState {
        left_pressed: bit_mask & 0b001 == bit_mask,
        middle_pressed: bit_mask & 0b010 == bit_mask,
        right_pressed: bit_mask & 0b100 == bit_mask,
    }
}

#[derive(Debug)]
/// The three buttons on the llemu
pub enum Button {
    /// The left button
    Left,
    /// The middle button
    Middle,
    /// The right button
    Right,
}

/// The callbacks for the three buttons on the llemu
struct ButtonCallbacks {
    /// The callback for when the left button is pressed
    pub left_cb: Option<Box<dyn Fn() + Send>>,
    /// The callback for when the middle button is pressed
    pub middle_cb: Option<Box<dyn Fn() + Send>>,
    /// The callback for when the right button is pressed
    pub right_cb: Option<Box<dyn Fn() + Send>>,
}

impl Debug for ButtonCallbacks {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ButtonCallbacks")
            .field("left_cb", &self.left_cb.is_some())
            .field("middle_cb", &self.middle_cb.is_some())
            .field("right_cb", &self.right_cb.is_some())
            .finish()
    }
}

lazy_static::lazy_static! {
    static ref BUTTON_CALLBACKS: Mutex<ButtonCallbacks> = Mutex::new(ButtonCallbacks {
        left_cb: None,
        middle_cb: None,
        right_cb: None,
    });
}

// this needs to return errors
/// Registers a callback for a button on the llemu
pub fn register(cb: impl Fn() + 'static + Send, button: Button) {
    unsafe {
        pros_sys::lcd_initialize();
    }

    extern "C" fn button_0_cb() {
        if let Some(cb) = &BUTTON_CALLBACKS.lock().left_cb {
            cb();
        }
    }

    extern "C" fn button_1_cb() {
        if let Some(cb) = &BUTTON_CALLBACKS.lock().middle_cb {
            cb();
        }
    }

    extern "C" fn button_2_cb() {
        if let Some(cb) = &BUTTON_CALLBACKS.lock().right_cb {
            cb();
        }
    }

    if !match button {
        Button::Left => {
            BUTTON_CALLBACKS.lock().left_cb = Some(Box::new(cb));
            unsafe { pros_sys::lcd_register_btn0_cb(Some(button_0_cb)) }
        }
        Button::Middle => {
            BUTTON_CALLBACKS.lock().middle_cb = Some(Box::new(cb));
            unsafe { pros_sys::lcd_register_btn1_cb(Some(button_1_cb)) }
        }
        Button::Right => {
            BUTTON_CALLBACKS.lock().right_cb = Some(Box::new(cb));
            unsafe { pros_sys::lcd_register_btn2_cb(Some(button_2_cb)) }
        }
    } {
        panic!("Setting button callback failed, even though lcd initialization was attempted.");
    }
}
