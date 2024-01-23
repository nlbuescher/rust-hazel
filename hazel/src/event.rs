use std::{
    fmt::Display,
    ops::{BitAnd, BitOr},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EventCategory(u8);

impl EventCategory {
    pub const None: EventCategory = EventCategory(0x00);
    pub const Application: EventCategory = EventCategory(0x01);
    pub const Input: EventCategory = EventCategory(0x02);
    pub const Keyboard: EventCategory = EventCategory(0x04);
    pub const Mouse: EventCategory = EventCategory(0x08);
}

impl BitAnd for EventCategory {
    type Output = EventCategory;

    fn bitand(self, rhs: Self) -> Self::Output {
        return EventCategory(self.0 & rhs.0);
    }
}

impl BitOr for EventCategory {
    type Output = EventCategory;

    fn bitor(self, rhs: Self) -> Self::Output {
        return EventCategory(self.0 | rhs.0);
    }
}

pub enum Event {
    WindowClose,
    WindowResize { width: u32, height: u32 },
    WindowFocus,
    WindowLostFocus,
    WindowMoved,

    AppTick,
    AppUpdate,
    AppRender,

    KeyPressed { key: u32, repeat_count: u32 },
    KeyReleased { key: u32 },

    MouseButtonPressed { button: u32 },
    MouseButtonReleased { button: u32 },
    MouseMoved { x: f32, y: f32 },
    MouseScrolled { x_offset: f32, y_offset: f32 },
}

impl Event {
    pub fn get_category(&self) -> EventCategory {
        match *self {
            Event::WindowClose
            | Event::WindowResize { .. }
            | Event::WindowFocus
            | Event::WindowLostFocus
            | Event::WindowMoved
            | Event::AppTick
            | Event::AppUpdate
            | Event::AppRender => EventCategory::Application,
            Event::KeyPressed { .. } | Event::KeyReleased { .. } => {
                EventCategory::Input | EventCategory::Keyboard
            }
            Event::MouseButtonPressed { .. }
            | Event::MouseButtonReleased { .. }
            | Event::MouseMoved { .. }
            | Event::MouseScrolled { .. } => EventCategory::Input | EventCategory::Mouse,
        }
    }

    pub fn is_in_category(&self, category: EventCategory) -> bool {
        (self.get_category() & category) != EventCategory::None
    }
}

impl Display for Event {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match *self {
            Event::WindowClose => String::from("WindowClose"),
            Event::WindowResize { width, height } => {
                format!("WindowResize: {}, {}", width, height)
            }
            Event::WindowFocus => String::from("WindowFocus"),
            Event::WindowLostFocus => String::from("WindowLostFocus"),
            Event::WindowMoved => String::from("WindowMoved"),
            Event::AppTick => String::from("AppTick"),
            Event::AppUpdate => String::from("AppUpdate"),
            Event::AppRender => String::from("AppRender"),
            Event::KeyPressed { key, repeat_count } => {
                format!("KeyPressed: {key} ({repeat_count} repeats)")
            }
            Event::KeyReleased { key } => {
                format!("KeyReleased: {key}")
            }
            Event::MouseButtonPressed { button } => {
                format!("MouseButtonPressed: {button}")
            }
            Event::MouseButtonReleased { button } => {
                format!("MouseButtonReleased: {button}")
            }
            Event::MouseMoved { x, y } => {
                format!("MouseMoved: {x}, {y}")
            }
            Event::MouseScrolled { x_offset, y_offset } => {
                format!("MouseScrolled: {x_offset}, {y_offset}")
            }
        };
        return fmt.pad(string.as_str());
    }
}
