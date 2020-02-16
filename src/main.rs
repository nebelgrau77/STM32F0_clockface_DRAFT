#![no_std]
#![no_main]
#![feature(type_ascription)]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate stm32f0xx_hal as hal;
extern crate embedded_hal;

use cortex_m_rt::entry;
use ssd1306::{prelude::*, Builder as SSD1306Builder};
use embedded_graphics::{image::Image, pixelcolor::BinaryColor, prelude::*};

use micromath::F32Ext;

use crate::hal::{
    prelude::*,
    stm32,
    i2c::I2c
    };

#[entry]
fn main() -> ! {

       if let (Some(mut p), Some(cp)) = (stm32::Peripherals::take(),cortex_m::peripheral::Peripherals::take()) {
      
        cortex_m::interrupt::free(move |cs| {

        let mut rcc = p.RCC.configure().sysclk(8.mhz()).freeze(&mut p.FLASH);
        
        let gpioa = p.GPIOA.split(&mut rcc);
        let scl = gpioa.pa9.into_alternate_af4(cs);
        let sda = gpioa.pa10.into_alternate_af4(cs);
        let i2c = I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), &mut rcc);
        
        // Set up the display                        
        let mut disp: GraphicsMode<_> = SSD1306Builder::new().size(DisplaySize::Display128x32).connect_i2c(i2c).into();

        disp.init().unwrap();
       
        let xcenter: f32 = 63.0;
        let ycenter: f32 = 15.0;
        let divider: f32 = 12.0;
        let radius: f32 = 14.0;
        
        
        for n in 0..12 {

            let (xcoord, ycoord) = coordinates(xcenter, ycenter, n as f32, divider, radius);
            
            let xc = xcoord as u32;
            let yc = ycoord as u32;

           
            disp.set_pixel(xc, yc, 1);
            

        }
        

        disp.flush().unwrap();

      
       
    });
    
}

    loop {continue;}
    
}


fn coordinates (center_x: f32, center_y: f32, n: f32, divider: f32, radius: f32) -> (f32,f32) {
    
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;

    let a: f32 = n*2.0*3.141596/divider;
    
    x: f32 = center_x + radius * a.cos();
    y: f32 = center_y + radius * a.sin();
    

    return (x, y);

}
