mod service;
use crate::service::api::Barco;

enum _Tablero {

}

fn main() {
    println!("Bienvenido al puto juego\n");
    let mut b1 = Barco {hundido : false, casillas : vec![12, -12, 8]};
    println!("{}", b1.explotar());
}
