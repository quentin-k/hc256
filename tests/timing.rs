use rand::prelude::*;
use std::time::Instant;
use hc256::Hc256;
use std::fs::File;
use std::io::Write;

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

    let mut file = File::create("init-time-info-2048").unwrap();
    file.write_all(&info_string.as_bytes()).unwrap();
}