use core::ptr::Unique;
use volatile::Volatile;
use core::fmt;
use spin::Mutex;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    col: usize,
    row: usize,
    color_code: ColorCode,
    buffer: Unique<Buffer>,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row;
                let col = self.col;

                let color_code = self.color_code;
                self.buffer().chars[row][col].write(ScreenChar{
                    ascii_character: byte,
                    color_code: color_code,
                });
                self.col += 1;
            }
        }
    }

    pub fn write_str(&mut self, s: &str)
    {
        for byte in s.bytes()
        {
            self.write_byte(byte);
        }
    }

    pub fn set_color(&mut self, foreground: Color, background: Color) {
        self.color_code = ColorCode::new(foreground, background);
    }

    pub fn clear_screen(&mut self)
    {
        let color_code = self.color_code;
        for i in 0..BUFFER_WIDTH {
            for j in 0..BUFFER_HEIGHT {
                self.buffer().chars[j][i].write(ScreenChar{
                    ascii_character: b' ',
                    color_code: color_code
                });
            }
        }
        self.row = 0;
        self.col = 0;
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe{ self.buffer.as_mut() }
    }

    fn new_line(&mut self) {
        self.col = 0;
        self.row += 1;
        let mut row = self.row;
        while row >= BUFFER_HEIGHT {
            for i in 0..BUFFER_WIDTH {
                for j in 1..BUFFER_HEIGHT {
                    let character = self.buffer().chars[j][i].read();
                    self.buffer().chars[j - 1][i].write(character);
                }
            }
            self.clear_row(BUFFER_HEIGHT - 1);
            row -= 1;
        }
        self.row = row;
    }

    fn clear_row(&mut self, row: usize)
    {
        let blank = ScreenChar {
        ascii_character: b' ',
        color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer().chars[row][col].write(blank);
        }
    }

}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
          self.write_byte(byte)
        }
        Ok(())
    }
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::vga_buffer::print(format_args!($($arg)*));
    });
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

pub fn clear_screen() {
    WRITER.lock().clear_screen();
}

pub fn print_literal_shit() {
    println!("Tos Kernel v0.1");
    println!("Hi Alex!");
    WRITER.lock().set_color(Color::Red, Color::Black);
    println!("Hi Joss, Tom, Nihal, Richard, Jacob, Alfie, Ross, Anthony, Dan, Lindzi");
    WRITER.lock().set_color(Color::White, Color::Black);
}



pub static WRITER:Mutex<Writer> = Mutex::new(Writer {
    col: 0,
    row: 0,
    color_code: ColorCode::new(Color::White, Color::Black),
    buffer: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
});
