use std::os::fd::AsRawFd;

const SEC: u32 = 1;
const MINUTE: u32 = 60 * SEC;
const HOUR: u32 = 60 * MINUTE;
const DAY: u32 = 24 * HOUR;
const WEEK: u32 = 7 * DAY;
const MONTH: u32 = 30 * DAY;
const YEAR: u32 = 365 * DAY;

trait UserLoad {
    fn load(&self, time: u32) -> u32;
}

struct TrapezLoad {
    max_concurrent_users: u32,
    end_down: u32,
    start_up: u32,
    slope: f32,
    plateau_start: u32,
    plateau_end: u32,
}

impl UserLoad for TrapezLoad {
    fn load(&self, time: u32) -> u32 {
        if time > self.end_down {
            0 // end is zero
        } else if time > self.plateau_end {
            self.max_concurrent_users - (time - self.plateau_end) * self.slope as u32
        } else if time > self.plateau_start {
            self.max_concurrent_users
        } else if time > self.start_up {
            (time - self.start_up) * self.slope as u32
        } else {
            0 // start is zero
        }
    }
}

fn build_load(max_concurrent_users: u32, duration: u32, plateau: u32) -> TrapezLoad {
    if plateau > duration {
        panic!("plateau must be smaller than duration");
    }

    let mid = 10*MINUTE / 2;
    let start_up = mid - (duration / 2);
    let end_down = mid + (duration / 2);
    let plateau_start = mid - (plateau / 2);
    let plateau_end = mid + (plateau / 2);
    let slope = max_concurrent_users as f32 / ((duration - plateau) / 2) as f32;

    TrapezLoad {
        max_concurrent_users: max_concurrent_users,
        end_down: end_down,
        start_up: start_up,
        slope: slope,
        plateau_start: plateau_start,
        plateau_end: plateau_end,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x() {
        let load = build_load(10, 5 * MINUTE, 1 * MINUTE);

        let mut sum = 0i64;
        let mut diff = 0i64;
        for i in 0..10*MINUTE {
            
            diff = load.load(i) as i64 -diff;
            println!("{} {}", i, diff);
        }
        println!("sum {}", diff);
    }
}
