/*
Copyright (c) 2020 Todd Stellanova
LICENSE: BSD3 (see LICENSE file)
*/

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_rtt_core::{self, rprintln, rtt_init_print};


use openmv_h7_bsp::board::Board;

use correlation_flow::micro_rfft;


static IMAGE0: &'static [u8] = include_bytes!("../testdata/64sq_250_30.gray");
static IMAGE1: &'static [u8] = include_bytes!("../testdata/64sq_255_33.gray");

#[entry]
fn main() -> ! {
    rtt_init_print!(NoBlockTrim);
    rprintln!("--> MAIN --");

    let mut mr_fft = micro_rfft::MicroFftContext::new();

    rprintln!("start flow calcs: {}", IMAGE0.len());

    const COLS: usize = 64;
    const ROWS: usize = 64;

    const FRAME_COUNT: u32 = 10;
    let mut last_flow = (0i16, 0i16);
    for _ in 0..FRAME_COUNT {
        last_flow = mr_fft.calculate_flow_fft(
            &IMAGE1,
            &IMAGE0,
            COLS as usize,
            ROWS as usize,
        );
        rprintln!("rfft flow: {:?}", last_flow);
    }
    rprintln!("mr_fft flow: {:?}", last_flow);
    //TODO use eg DWT::get_cycle_count() to measure elapsed time

    rprintln!("<--- DONE --");

    loop {

    }


}