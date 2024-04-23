mod microcontroller;
mod connection;
mod bash;

use microcontroller::handle_microcontroller;
use connection::connect;
use bash::execute_script;

fn main() {
    match handle_microcontroller(execute_script()) {
        Ok(_) => {
            let devices = load_devices(); 
            match connect(devices) {
                Ok(_) => println!("Conexión exitosa"),
                Err(e) => println!("Error al conectar: {:?}", e),
            }
        },
        Err(e) => println!("Error en el manejo del microcontrolador: {:?}", e),
    }
}

fn load_devices() -> Vec<Device> {
    vec![]
}
