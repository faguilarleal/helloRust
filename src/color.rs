// color.rs
#[derive(Debug, PartialEq, Eq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    pub fn red(&self) -> u8 {
        self.red
    }

    pub fn green(&self) -> u8 {
        self.green
    }

    pub fn blue(&self) -> u8 {
        self.blue
    }

    pub fn set_red(&mut self, red: u8) {
        self.red = red;
    }

    pub fn set_green(&mut self, green: u8) {
        self.green = green;
    }

    pub fn set_blue(&mut self, blue: u8) {
        self.blue = blue;
    }

    // Métodos para obtener los componentes de color para BMP
    pub fn r(&self) -> u8 {
        self.red
    }

    pub fn g(&self) -> u8 {
        self.green
    }

    pub fn b(&self) -> u8 {
        self.blue
    }
}
