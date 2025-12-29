pub struct FreeRtosInstant {
    sys_tick: u32,
    count: u32,
}

impl FreeRtosInstant {
    fn new(sys_tick: u32, count: u32) -> Self {
        Self { sys_tick, count }
    }

    fn reload_value() -> u64 {
        9 + 1
    }

    fn add(sys_tick: u32, count: u32, tick: u64) -> (u32, u32) {
        let reload = Self::reload_value();
        let diff_count = tick / reload;
        let diff_sys_tick = (tick - diff_count * reload) as u32;
        let mut diff_count = diff_count as u32;
        let new_sys_tick = if diff_sys_tick > sys_tick {
            diff_count += 1;
            sys_tick + reload as u32 - diff_sys_tick
        } else {
            sys_tick - diff_sys_tick
        };
        (new_sys_tick, count.wrapping_add(diff_count))
    }

    fn elapsed(&mut self, sys_tick: u32, count: u32) -> u64 {
        let reload = Self::reload_value();
        let diff = count.wrapping_sub(self.count) as u64;
        let t = diff * reload + self.sys_tick as u64 - sys_tick as u64;
        t
    }
}

#[test]
fn tick_test() {
    assert_eq!(FreeRtosInstant::add(9, 0, 3), (6, 0));
    assert_eq!(FreeRtosInstant::add(9, 0, 9), (0, 0));
    assert_eq!(FreeRtosInstant::add(9, 0, 10), (9, 1));
    assert_eq!(FreeRtosInstant::add(9, 0, 19), (0, 1));
    assert_eq!(FreeRtosInstant::add(9, 0, 50), (9, 5));
    assert_eq!(FreeRtosInstant::add(0, 0, 1), (9, 1));
    assert_eq!(FreeRtosInstant::add(0, 0, 19), (1, 2));

    let mut i = FreeRtosInstant::new(9, 0);
    assert_eq!(i.elapsed(4, 0), 5);
    assert_eq!(i.elapsed(0, 0), 9);
    let mut i = FreeRtosInstant::new(5, 0);
    assert_eq!(i.elapsed(9, 1), 6);
    assert_eq!(i.elapsed(0, 1), 15);
    assert_eq!(i.elapsed(6, 10), 99);
}
