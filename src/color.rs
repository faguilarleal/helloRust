// color.rs
#[derive(Debug, PartialEq, Eq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    // Constructor para crear un nuevo color
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    // Método para obtener el valor rojo
    fn red(&self) -> u8 {
        self.red
    }

    // Método para obtener el valor verde
    fn green(&self) -> u8 {
        self.green
    }

    // Método para obtener el valor azul
    fn blue(&self) -> u8 {
        self.blue
    }

    // Método para establecer el valor rojo
    pub fn set_red(&mut self, red: u8) {
        self.red = red;
    }

    // Método para establecer el valor verde
    pub fn set_green(&mut self, green: u8) {
        self.green = green;
    }

    // Método para establecer el valor azul
    pub fn set_blue(&mut self, blue: u8) {
        self.blue = blue;
    }
}
