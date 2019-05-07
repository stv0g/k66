#![no_std]
#![no_main]

#[macro_use]
extern crate lazy_static;
extern crate panic_halt;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use k66::lptmr0::psr::PCSW;
use k66::GPIOE;
use k66::LPTMR0;
use k66::{interrupt, Interrupt, Peripherals};

lazy_static! {
    static ref MUTEX_GPIOE_PTOR: Mutex<RefCell<Option<GPIOE>>> = Mutex::new(RefCell::new(None));
    static ref MUTEX_LPTRM_CSR: Mutex<RefCell<Option<LPTMR0>>> = Mutex::new(RefCell::new(None));
}

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let sim = &p.SIM;
    sim.scgc5
        .modify(|_, w| w.porte().set_bit().lptmr().set_bit());

    let lptmr0 = &p.LPTMR0;
    lptmr0
        .psr
        .modify(|_, w| w.pcs().variant(PCSW::_01).pbyp().set_bit());
    lptmr0
        .cmr
        .modify(|_, w| unsafe { w.compare().bits(500 - 1) });
    lptmr0.csr.modify(|_, w| w.tie().set_bit().ten().set_bit());

    let porte = &p.PORTE;
    porte.pcr26.modify(|_, w| w.mux()._001());
    let gpioe = &p.GPIOE;
    gpioe.pddr.modify(|_, w| w.pdd26().set_bit());

    cortex_m::interrupt::free(|cs| {
        MUTEX_GPIOE_PTOR.borrow(cs).replace(Some(p.GPIOE));
        MUTEX_LPTRM_CSR.borrow(cs).replace(Some(p.LPTMR0));
    });

    config_nvic();

    loop {}
}

#[interrupt]
fn LPTMR0() {
    cortex_m::interrupt::free(|cs| {
        let ref_cell = MUTEX_GPIOE_PTOR.borrow(cs).borrow();
        if let Some(gpioe) = ref_cell.as_ref() {
            gpioe.ptor.write(|w| w.ptto26().set_bit());
        }
    });

    cortex_m::interrupt::free(|cs| {
        let ref_cell = MUTEX_LPTRM_CSR.borrow(cs).borrow();
        if let Some(lptmr) = ref_cell.as_ref() {
            lptmr.csr.modify(|_, w| w.tcf().set_bit());
        }
    });
}

fn config_nvic() {
    let mut p = cortex_m::Peripherals::take().unwrap();
    p.NVIC.enable(Interrupt::LPTMR0);
}
