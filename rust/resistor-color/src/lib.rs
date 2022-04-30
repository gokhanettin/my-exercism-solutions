use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[repr(usize)]
#[derive(Debug, PartialEq, Clone, Copy, IntoEnumIterator, IntEnum)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(color: ResistorColor) -> usize {
    color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {
    if let Ok(color) = ResistorColor::from_int(value) {
        format!("{:?}", color)
    } else {
        String::from("value out of range")
    }
}

pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().collect()
}
