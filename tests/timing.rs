use rand::prelude::*;
use std::time::Instant;
use hc256::Hc256;
use std::fs::File;
use std::io::Write;

// Allow measurement of clock cycles
mod ffi {
    extern {
        pub fn clock() -> ::libc::clock_t;
    }
}

#[test]
fn initialize_time() {
    let mut keys: [[u8;32]; 2048] = [[0;32]; 2048];
    let mut ivs: [[u8;32]; 2048] = [[0;32]; 2048];

    for i in 0..2048 {
        rand::thread_rng().fill_bytes(&mut keys[i]);
        rand::thread_rng().fill_bytes(&mut ivs[i]);
    }

    let mut shortest = u128::MAX;
    let mut longest = u128::MIN;
    let mut init_sum = 0;

    // Warmup
    for i in 0..2048 {
        Hc256::new(&keys[i], &ivs[i]);
    }

    // Run test
    let total_time = Instant::now();
    for i in 0..2048 {
        let init_time = Instant::now();
        Hc256::new(&keys[i], &ivs[i]);
        let elapsed = init_time.elapsed().as_nanos();

        if elapsed > longest {
            longest = elapsed;
        }

        if elapsed < shortest {
            shortest = elapsed;
        }

        init_sum += elapsed;
    }
    let total_time = total_time.elapsed().as_nanos();
    let extras = total_time - init_sum;
    let avg_init = init_sum as f64 / 2048.0;
    let avg_cycle = total_time as f64 / 2048.0;
    let max_diff = longest - shortest;
    let highest_from_avg = longest as f64 - avg_init;
    let lowest_from_avg = avg_init - shortest as f64;

    let info_string = format!(
        "-------------------------------------\nData\n-------------------------------------\nTotal time:           {}\nTotal inits:          {}\nWasted loop time:     {}\nAverage init time:    {}\nAverage cycle time:   {}\nFastest init time:    {}\nSlowest init time:    {}\nMax diff:             {}\nSlowest from average: {}\nFastest from average: {}\n-------------------------------------\n"
        , total_time
        , init_sum
        , extras
        , avg_init
        , avg_cycle
        , shortest
        , longest
        , max_diff
        , highest_from_avg
        , lowest_from_avg
    );

    std::fs::create_dir_all("timing").unwrap();
    let mut file = File::create("timing/init-time-info-2048").unwrap();
    file.write_all(&info_string.as_bytes()).unwrap();
}

#[test]
fn apply_stream_time() {
    let key: [u8;32] = [0;32];
    let iv: [u8;32] = [0;32];

    let mut cipher = Hc256::new(&key, &iv);

    let mut data: [u8; 16384] = [0;16384];

    let mut shortest = u128::MAX;
    let mut longest = u128::MIN;
    let mut stream_sum = 0;

    // Warmup
    for _ in 0..10000 {
        cipher.apply_stream(&mut data);
    }

    // Run test
    let total_time = Instant::now();
    for _ in 0..262144 {
        let init_time = Instant::now();
        cipher.apply_stream(&mut data);
        let elapsed = init_time.elapsed().as_nanos();

        if elapsed > longest {
            longest = elapsed;
        }

        if elapsed < shortest {
            shortest = elapsed;
        }

        stream_sum += elapsed;
    }
    let total_time = total_time.elapsed().as_nanos();
    let extras = total_time - stream_sum;
    let avg_stream = stream_sum as f64 / 262144.0;
    let avg_cycle = total_time as f64 / 262144.0;
    let max_diff = longest - shortest;
    let highest_from_avg = longest as f64 - avg_stream;
    let lowest_from_avg = avg_stream - shortest as f64;

    let info_string = format!(
        "-------------------------------------\nData\n-------------------------------------\nTotal time:           {}\nTotal stream:         {}\nWasted loop time:     {}\nAverage stream time:  {}\nAverage cycle time:   {}\nFastest stream time:  {}\nSlowest stream time:  {}\nMax diff:             {}\nSlowest from average: {}\nFastest from average: {}\n-------------------------------------\n"
        , total_time
        , stream_sum
        , extras
        , avg_stream
        , avg_cycle
        , shortest
        , longest
        , max_diff
        , highest_from_avg
        , lowest_from_avg
    );

    std::fs::create_dir_all("timing").unwrap();
    let mut file = File::create("timing/apply-stream-time-info-4_3GB").unwrap();
    file.write_all(&info_string.as_bytes()).unwrap();
    let nspb = avg_stream / 16384f64;
    assert!( nspb <= 12.0)
}

#[test]
fn clock_stream_time() {
    let mut cipher = Hc256::new(&[0;32], &[0;32]);
    let mut data = [0u8; 64];
    let start = unsafe { ffi::clock() };
    for _ in 0..0x4000000 {
        cipher.apply_stream(&mut data);
    }
    let finish = unsafe { ffi::clock() };
    let num_cycles = (finish - start) as u64;
    let cpb =  num_cycles as f64 / (0x4000000u64 * 64) as f64;

    let info_string = format!(
        "-------------------------------------\nData\n-------------------------------------\nStart:                   {}\nFinish:                  {}\nNum cycles:              {}\nAverage cycles per byte: {}\n-------------------------------------\n"
        , start
        , finish
        , num_cycles
        , cpb
    );

    std::fs::create_dir_all("timing").unwrap();
    let mut file = File::create("timing/clock-stream-time").unwrap();
    file.write_all(&info_string.as_bytes()).unwrap();
}