mod color; // con esto exporto el modulo de color

use color::Color; // importa la estructura color del modulo color. 

fn main(){
    let red = Color::new(255, 0, 0);
    println!("Este es mi color: {:#?}",red);
}