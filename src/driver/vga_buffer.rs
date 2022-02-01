use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black      = 0x0,
    Blue       = 0x1,
    Green      = 0x2,
    Cyan       = 0x3,
    Red        = 0x4,
    Magenta    = 0x5,
    Brown      = 0x6,
    LightGray  = 0x7,
    DarkGray   = 0x8,
    LightBlue  = 0x9,
    LightGreen = 0xA,
    LightCyan  = 0xB,
    LightRed   = 0xC,
    Pink       = 0xD,
    Yellow     = 0xE,
    White      = 0xF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorInformation(u8);

impl ColorInformation {
    pub fn new(foreground: Color, background: Color) -> ColorInformation {
        ColorInformation((background as u8) << 4 | foreground as u8)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    ascii_character: u8,
    color_information: ColorInformation
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub struct Writer {
    cursor_position: (usize, usize),
    color_information: ColorInformation,
    buffer: &'static mut Buffer
}

impl Writer {
    pub fn write_screen_char(&mut self, character: ScreenChar) {
        if self.cursor_position.0 >= BUFFER_WIDTH {
            self.new_line();
        }
        if self.cursor_position.1 >= BUFFER_WIDTH {
            self.scroll();
        }
        self.buffer.chars[self.cursor_position.1][self.cursor_position.0].write(character);
        self.cursor_position.0 += 1;
    }
    pub fn write_ascii_byte(&mut self, character: u8) {
        match character {
            b'\n' => self.new_line(),
            character => {
                self.write_screen_char(ScreenChar { ascii_character: character, color_information: self.color_information })
            }
        }
    }
    pub fn new_line(&mut self) {
        self.cursor_position.1 += 1;
        self.cursor_position.0 = 0;
    }
    pub fn scroll(&mut self) {
        self.cursor_position.1 -= 1;
        for y in 1..BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH {
                self.buffer.chars[y-1][x].write(self.buffer.chars[y][x].read());
            }
        }
        self.clear_line(BUFFER_HEIGHT-1);
    }
    pub fn clear_line(&mut self, line: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_information: self.color_information
        };
        for x in 0..BUFFER_WIDTH {
            self.buffer.chars[line][x].write(blank);
        }
    }
}

impl core::fmt::Write for Writer {
    fn write_char(&mut self, c: char) -> core::fmt::Result {
        let character = c as u8;
        match character {
            0x20..=0x7E | b'\n' => self.write_ascii_byte(character),
            _ => self.write_ascii_byte(0xFE)
        }
        Ok(())
    }
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for character in s.chars() {
            self.write_char(character)?;
        }
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer { 
        color_information: ColorInformation::new(Color::White, Color::Black), 
        cursor_position: (0,0), 
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) } 
    });
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

