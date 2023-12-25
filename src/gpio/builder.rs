use super::{Gpio, Port, Pin, PinMode};

pub struct GpioBuilder {
    gpio: Gpio,
}

impl GpioBuilder {
    pub fn new(port: Port, pin: Pin) -> Self {
        Self {
            gpio: Gpio::new(port, pin) 
        }
    }

    pub fn mode(mut self, mode: PinMode) -> Self {
        self.gpio.set_mode(mode);
        self
    }
    
    pub fn build(self) -> Gpio {
        self.gpio
    }
}
