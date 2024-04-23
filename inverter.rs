use rand::Rng;
use std::time::Duration;
use tokio::time::interval;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
struct ConfigConnection {
    baud_rate: u32,
    parity: String,
    data_bits: u8,
    stop_bits: u8,
}

impl ConfigConnection {
    fn new(baud_rate: u32, parity: String, data_bits: u8, stop_bits: u8) -> Self {
        Self {
            baud_rate,
            parity,
            data_bits,
            stop_bits,
        }
    }

    fn get_config(&self) -> Self {
        self.clone()
    }
}

#[derive(Clone)]
struct ConfigInverter {
    server_id: u32,
    address: u32,
    registers: u32,
    sample_time: u32,
}

impl ConfigInverter {
    fn new(server_id: u32, address: u32, registers: u32, sample_time: u32) -> Self {
        Self {
            server_id,
            address,
            registers,
            sample_time,
        }
    }

    fn get_info(&self) -> Self {
        self.clone()
    }

    fn set_info(&mut self, server_id: u32, sample_time: u32) {
        self.server_id = server_id;
        self.sample_time = sample_time;
    }
}

struct Inverter {
    id: u32,
    port_device: String,
    parameters: ConfigConnection,
    config: ConfigInverter,
    token: String,
}

impl Inverter {
    fn new(port_device: String) -> Self {
        let id = rand::thread_rng().gen_range(0..1_000_000_000);
        let parameters = ConfigConnection::new(9600, "none".to_string(), 8, 1);
        let config = ConfigInverter::new(1, 3000, 83, 1000);
        let token = if port_device == "/dev/ttyUSB0" {
            "bUArsYQI9E0rgkNxrm63".to_string()
        } else {
            "wZQTwoKjKMK3tSbi4CvN".to_string()
        };

        Self {
            id,
            port_device,
            parameters,
            config,
            token,
        }
    }

    async fn get_data(&self, client: Arc<Mutex<dyn ModbusClient>>) {
        let mut interval = interval(Duration::from_millis(self.config.sample_time.into()));
        loop {
            interval.tick().await;
            let mut client = client.lock().await;
            let result = client.read_input_registers(self.config.address, self.config.registers).await;

            match result {
                Ok(data) => {
                    let mut pack_modbus = vec![];
                    for i in 0..self.config.registers as usize {
                        pack_modbus.push(data[i]);
                    }

                },
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
            }
        }
    }

    fn set_parameters(&mut self, baud_rate: u32, parity: String, data_bits: u8, stop_bits: u8) {
        self.parameters = ConfigConnection::new(baud_rate, parity, data_bits, stop_bits);
    }

    fn set_config(&mut self, server_id: u32, address: u32, registers: u32, sample_time: u32) {
        self.config = ConfigInverter::new(server_id, address, registers, sample_time);
    }
}

trait ModbusClient {
    fn read_input_registers(&mut self, address: u32, quantity: u32) -> Result<Vec<u16>, String>;
}

