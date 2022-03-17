#![allow(unused)]

fn main() {
    println!("Hello, world!");
}

// 16-bit, 32 khz
struct Oscillator {
    amplitude: i16,
    period: u32,
    phase: u32,
    pulse_width: u32,
    rise_time: u32, // 0 for sawtooth, period / 2 for triangle
    squareness: i16, // 0 for saw/tri, amplitude for square
}

// rise_time = 0 = sawtooth
//
//  |\
//  | \
// ----------------------------
//      \ |
//       \|

// rise_time = period / 2 = triangle
//
//   /\
//  /  \
// ----------------------------
//       \  /
//        \/


impl Oscillator {
    fn get_sample(&self, offset: u32) -> i16 {
        assert!(self.rise_time <= self.period / 2);

        let osc_offset = offset % self.period;
        let half_period = self.period / 2;
        let half_rise_time = self.rise_time / 2;
        let in_initial_rise = osc_offset < half_rise_time;
        let in_final_rise = osc_offset > self.period - half_rise_time;
        let in_fall = !in_initial_rise && !in_final_rise;
        let fall_time = self.period - self.rise_time;
        let half_fall_time = fall_time / 2;

        let amplitude_i32: i32 = self.amplitude.into();
        let rise_time_i32: i32 = self.rise_time.try_into().expect("overflow");
        let half_rise_time_i32: i32 = half_rise_time.try_into().expect("overflow");
        let osc_offset_i32: i32 = osc_offset.try_into().expect("overflow");
        let fall_time_i32: i32 = fall_time.try_into().expect("overflow");
        let half_fall_time_i32: i32 = half_fall_time.try_into().expect("overflow");

        if in_initial_rise {
            // delta = amplitude / half_rise_time
            // sample = delta * osc_offset
            let sample = amplitude_i32.saturating_mul(osc_offset_i32) / half_rise_time_i32;
            sample.try_into().expect("overflow")
        } else if in_fall {
            let working_offset = osc_offset_i32 - half_rise_time_i32;
            // delta = amplitude / half_fall_time
            // sample = amplitude - delta * working_offset
            let sample = amplitude_i32 - amplitude_i32.saturating_mul(working_offset) / half_fall_time_i32;
            sample.try_into().expect("overflow")
        } else if in_final_rise {
            let working_offset = osc_offset_i32 - half_rise_time_i32 - fall_time_i32;
            
            let sample = amplitude_i32.saturating_mul(working_offset) / half_rise_time_i32 - amplitude_i32;
            sample.try_into().expect("overflow")
        } else {
            unreachable!()
        }       
    }
}

