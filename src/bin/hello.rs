#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

use core::alloc::Layout;
use core::borrow::BorrowMut;
use core::cell::RefCell;
use core::convert::TryFrom;
use core::str::Bytes;

#[macro_use(singleton)]
use cortex_m;

use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};

use cortex_m::interrupt::InterruptNumber;
use cortex_m::singleton;
use cortex_m_rt::{exception, ExceptionFrame};
use freertos_rust::*;
use freertos_rust::{FreeRtosCharPtr, FreeRtosTaskHandle};


use stm32f1xx_hal::can::Can;
use stm32f1xx_hal::device::{CAN1, SPI1, SPI2, TIM2};
use stm32f1xx_hal::dma::{RxTxDma, TxDma};
use stm32f1xx_hal::gpio::{Alternate, Floating, Input, OutputSpeed, Pin, PushPull, CRH, CRL};
use stm32f1xx_hal::spi::{Mode, NoMiso, Phase, Polarity, Spi, Spi1NoRemap, Spi2NoRemap};
use stm32f1xx_hal::time::Hertz;
use stm32f1xx_hal::timer::{CountDownTimer, Event};
use stm32f1xx_hal::watchdog::IndependentWatchdog;
use stm32f1xx_hal::{delay::Delay, pac, pac::interrupt, prelude::*}; // STM32F1 specific functions
use stm32f1xx_hal::pac::rcc::csr::CSR_SPEC;

use embedded_hal::digital::v2::OutputPin;


#[global_allocator]
static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;

// define what happens in an Out Of Memory (OOM) condition
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    defmt::panic!();
}

#[allow(non_snake_case)]
#[no_mangle]
fn vApplicationStackOverflowHook(pxTask: FreeRtosTaskHandle, pcTaskName: FreeRtosCharPtr) {
    defmt::panic!("Application stack overflow");
}


#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello STM32");
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut reset_flags = &dp.RCC.csr;

    let reset_reason = hello::reset_reason::reset_reason(&reset_flags);
    
    let rcc = dp.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(72.mhz())
        .hclk(64.mhz())
        .pclk1(36.mhz())
        .pclk2(64.mhz())
        .freeze(&mut flash.acr);
   

    let mut nvic = cp.NVIC;

    unsafe {
        // Set the interrupt priorities as needed by FreeRTOS.  These are raw
        // values set into the register.
        // http://www.FreeRTOS.org/RTOS-Cortex-M3-M4.html
        // If you face unexplained lockups and crashes, badly configured
        // interrupt priority is a likely cause.

        // Priority 12, 0xC0.  The lower bits must be 1, which makes it 207
        nvic.set_priority(stm32f1xx_hal::pac::Interrupt::TIM2, 207);
        // Priority 13, 0xD0.  The lower bits must be 1, which makes it 223;
        nvic.set_priority(stm32f1xx_hal::pac::Interrupt::USB_LP_CAN_RX0, 223);
    }

    // Start of Freertos Tasks
    let hello_task = Task::new()
        .name("hlo")
        .stack_size(128)
        .priority(TaskPriority(1))
        .start(move || {
            defmt::println!("Hello Task started");
            
            loop {

                freertos_rust::CurrentTask::delay(Duration::ms(100));
                defmt::println!("h");
            }
        })
        .unwrap();

    // reporting task
    let report = Task::new()
        .name("rep")
        .stack_size(128)
        .priority(TaskPriority(4))
        .start(move || {
            loop {
            
                freertos_rust::CurrentTask::delay(Duration::ms(1000));
                defmt::println!("1 sec tick");
            
            }
        });    
    FreeRtosUtils::start_scheduler();
    // FreeRTOS runs after this.
}

