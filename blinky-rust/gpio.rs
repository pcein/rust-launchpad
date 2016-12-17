#![allow(dead_code)]

pub const SYSCTL_PERIPH_GPIOF: u32 = 0xf0000805;
pub const GPIO_PORTF_BASE: u32 = 0x40025000;

pub const GPIO_PIN_1: u32 = (1 << 1);
pub const GPIO_PIN_2: u32 = (1 << 2);

extern {
    fn SysCtlPeripheralEnable(p: u32);
    fn GPIOPinTypeGPIOOutput(base: *const u32, mask: u32);
    fn GPIOPinWrite(base: *const u32, pin: u32, output_val: u32);
}

// Enable PORTF
pub fn port_enable(peripheral: u32)
{
    unsafe {
        SysCtlPeripheralEnable(peripheral);
    }
}

pub fn port_output(port: u32, pin: u32) {
    unsafe {
        GPIOPinTypeGPIOOutput(port as *const u32, pin);
    }
}

pub fn port_write(port: u32, pin: u32, value: u32) {
    unsafe {
        GPIOPinWrite(port as *const u32, pin, value);
    }
}
