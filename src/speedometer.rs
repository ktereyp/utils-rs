use std::time::SystemTime;

const SLOT: usize = 6;

#[derive(Default, Copy, Clone)]
struct SpeedRecord {
    time: u64,
    value: usize,
}

#[derive(Default, Copy, Clone)]
struct Speedometer {
    data: [SpeedRecord; SLOT],
}

impl Speedometer {
    fn new() -> Speedometer {
        Speedometer {
            data: [SpeedRecord::default(); SLOT],
        }
    }

    fn store(&mut self, change: usize) {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let slot = (now as usize % SLOT);
        if self.data[slot].time == now {
            self.data[slot].value += change;
        } else {
            self.data[slot].value = change;
            self.data[slot].time = now;
        }
    }

    fn speed(&self) -> usize {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

        let sum = self.data.iter().fold((0, 0), |sum, iter| {
            if iter.time != now && iter.time + 5 >= now {
                (sum.0 + iter.value, sum.1 + 1)
            } else {
                sum
            }
        });

        if sum.1 > 0 {
            sum.0 / sum.1
        } else {
            0
        }
    }
}
