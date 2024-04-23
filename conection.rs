use tokio_modbus::prelude::*;
use tokio_serial::{Serial, SerialPortSettings};
use rand::{thread_rng, Rng};
use std::io;

mod inverter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let devices = vec![]; 
    let mut inverters = vec![];

    for item in devices {
        let port_settings = SerialPortSettings {
            baud_rate: 19200,
            ..Default::default()
        };

        let port_name = "/dev/ttyUSB0"; 
        let port = Serial::from_path(port_name, &port_settings)?;

        let mut ctx = tokio_modbus::client::rtu::connect_slave(port, Slave(1)).await?;

        inverters.push({
            id: thread_rng().gen_range(0..100000000),
            name: String::from("Inversor X"),
            inverter: inverter::Inverter::new(item),
            clients_modbus: ctx,
        });
    }

    for inverter in inverters.iter_mut() {
        match inverter.clients_modbus.read_input_registers(0x0000, 7).await {
            Ok(buf) => println!("Received data: {:?}", buf),
            Err(e) => {
                println!("Error al conectar: {}", e);
                std::process::exit(1);
            }
        };
    }

    Ok(())
}
