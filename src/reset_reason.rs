use stm32f1xx_hal::pac::rcc::csr::CSR_SPEC;

pub enum ResetReason {
    PowerOnReset,
    PINReset,
    WatchdogReset,
    SWReset,
}

pub fn reset_reason(reset_flags: &stm32f1::Reg<CSR_SPEC>) -> ResetReason {

    let flags = reset_flags.read();
     if flags.iwdgrstf().is_reset() {
        //defmt::println!("Watchdog Reset occured");
        reset_flags.write(|w| w.iwdgrstf().clear_bit());
        return ResetReason::WatchdogReset;
    }
    if flags.wwdgrstf().is_reset() {
        //defmt::println!("Window Watchdog Reset occured");
        reset_flags.write(|w| w.wwdgrstf().clear_bit());
        return ResetReason::WatchdogReset;
    }
    if flags.porrstf().is_reset() {
        //defmt::println!("POR Reset occured");
        reset_flags.write(|w| w.porrstf().clear_bit());
        return ResetReason::PowerOnReset;
    }
    if flags.pinrstf().is_reset() {
        //defmt::println!("PIN Reset occured");
        reset_flags.write(|w| w.pinrstf().clear_bit());
        return ResetReason::PINReset;
    }
    if flags.sftrstf().is_reset() {
        //defmt::println!("SW Reset occured");
        reset_flags.write(|w| w.sftrstf().clear_bit());
        return ResetReason::SWReset;
    }
  
    if flags.lpwrrstf().is_reset() {
        defmt::println!("Low power Reset occured");
        reset_flags.write(|w| w.lpwrrstf().clear_bit());
        defmt::println!("SW Reset occured");
    }

    panic!("Unknown reset reason");
    
}