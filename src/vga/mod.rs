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

#[repr(transparent)]
pub struct Buffer<const height: usize, const width: usize> {
    chars: [[ScreenChar; width]; height]
}

pub struct Writer<const height: usize, const width: usize> {
    cursor_position: (usize, usize),
    color_information: ColorInformation,
    buffer: &'static mut Buffer<height, width>
}

impl<const height: usize, const width: usize> Writer<height, width> {
    pub fn write_screen_char(&mut self, character: ScreenChar) {
        if self.cursor_position.0 >= width {
            self.new_line();
        }
        if self.cursor_position.1 >= width {
            return;
        }
        self.buffer.chars[self.cursor_position.1][self.cursor_position.0] = character;
        self.cursor_position.0 += 1;
    }
    pub fn write_char(&mut self, character: u8) {
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
}

impl<const height: usize, const width: usize> Writer<height, width> {
    fn write(&mut self, string: &str) {
        for character in string.bytes() {
            match character {
                0x20..=0x7E | b'\n' => self.write_char(character),
                _ => self.write_char(0xFE)
            }
        }
    }
    pub fn writeln(&mut self, string: &str) {
        self.write(string);
        self.new_line();
    }
}

impl<const height: usize, const width: usize> Writer<height, width> {
    pub fn new(color_information: ColorInformation) -> Self {
        Writer { 
            color_information, 
            cursor_position: (0,0), 
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer<20, 80>) } 
        }
    }
}

