use crate::*;
use core::{cell::UnsafeCell, marker::PhantomData};
use os_trait::{FakeRawMutex, MicrosDurationU32, prelude::*};

/// `OsInterface` implementation, the N can be choose between [`SemaphoreNotifier`]
pub struct FreeRTOS<N> {
    _n: PhantomData<N>,
}

unsafe impl<N> Send for FreeRTOS<N> {}
unsafe impl<N> Sync for FreeRTOS<N> {}

impl<N> OsInterface for FreeRTOS<N>
where
    N: NotifyBuilder + 'static,
{
    type RawMutex = FakeRawMutex;
    type Notifier = N::Notifier;
    type NotifyWaiter = N::Waiter;
    type Timeout = FreeRtosTimeoutNs;
    type TimeoutState = FreeRtosTimeoutState;
    type Delay = FreeRtosTickDelayNs;

    const O: Self = Self { _n: PhantomData };

    #[inline]
    fn yield_thread() {
        CurrentTask::yield_now();
    }

    #[inline]
    fn timeout() -> Self::Timeout {
        FreeRtosTimeoutNs::new()
    }

    #[inline]
    fn delay() -> Self::Delay {
        FreeRtosTickDelayNs::new()
    }

    #[inline]
    fn notify() -> (Self::Notifier, Self::NotifyWaiter) {
        N::build()
    }
}

pub trait NotifyBuilder {
    type Notifier: Notifier;
    type Waiter: NotifyWaiter;

    fn build() -> (Self::Notifier, Self::Waiter);
}

/// `NotifyBuilder` implementation for task notification
#[derive(Clone)]
pub struct TaskNotifier {
    inner: Arc<UnsafeCell<Task>>,
}

unsafe impl Send for TaskNotifier {}
unsafe impl Sync for TaskNotifier {}

impl TaskNotifier {
    pub fn new() -> (Self, TaskNotifyWaiter) {
        let task = unsafe { Task::from_raw_handle(core::ptr::null()) };
        let inner = Arc::new(UnsafeCell::new(task));
        let inner2 = Arc::clone(&inner);
        (Self { inner }, TaskNotifyWaiter { inner: inner2 })
    }

    fn get_inner(&self) -> &mut Task {
        unsafe { &mut *self.inner.get() }
    }
}

impl NotifyBuilder for TaskNotifier {
    type Notifier = TaskNotifier;
    type Waiter = TaskNotifyWaiter;

    fn build() -> (Self::Notifier, Self::Waiter) {
        Self::new()
    }
}

impl Notifier for TaskNotifier {
    fn notify(&self) -> bool {
        let inner = self.get_inner();
        if inner.is_null() {
            return false;
        }

        if is_in_isr() {
            let mut ctx = InterruptContext::new();
            inner
                .notify_from_isr(&mut ctx, TaskNotification::OverwriteValue(1))
                .is_ok()
        } else {
            inner.set_notification_value(1);
            true
        }
    }
}

pub struct TaskNotifyWaiter {
    inner: Arc<UnsafeCell<Task>>,
}

impl TaskNotifyWaiter {
    fn get_inner(&self) -> &mut Task {
        unsafe { &mut *self.inner.get() }
    }
}

unsafe impl Send for TaskNotifyWaiter {}

impl NotifyWaiter for TaskNotifyWaiter {
    fn wait(&self, timeout: MicrosDurationU32) -> bool {
        let inner = self.get_inner();

        if let Ok(task) = Task::current() {
            if *inner != task {
                critical_section::with(|_| {
                    if *inner != task {
                        *inner = task;
                    }
                });
            }
        }

        if inner.is_null() {
            return false;
        }

        let dur = Duration::ms(timeout.to_millis());
        if let Ok(val) = inner.wait_for_notification(0, u32::MAX, dur) {
            return val != 0;
        }
        false
    }
}

/// `NotifyBuilder` implementation for [`Semaphore`]
#[derive(Clone)]
pub struct SemaphoreNotifier {
    inner: Arc<Semaphore>,
}

unsafe impl Send for SemaphoreNotifier {}

impl SemaphoreNotifier {
    pub fn new() -> (Self, SemaphoreNotifyWaiter) {
        let inner = Arc::new(Semaphore::new_binary().unwrap());
        let inner2 = Arc::clone(&inner);
        (Self { inner }, SemaphoreNotifyWaiter { inner: inner2 })
    }
}

impl NotifyBuilder for SemaphoreNotifier {
    type Notifier = SemaphoreNotifier;
    type Waiter = SemaphoreNotifyWaiter;

    fn build() -> (Self::Notifier, Self::Waiter) {
        Self::new()
    }
}

impl Notifier for SemaphoreNotifier {
    fn notify(&self) -> bool {
        if is_in_isr() {
            let mut ctx = InterruptContext::new();
            self.inner.give_from_isr(&mut ctx)
        } else {
            self.inner.give()
        }
    }
}

pub struct SemaphoreNotifyWaiter {
    inner: Arc<Semaphore>,
}

unsafe impl Send for SemaphoreNotifyWaiter {}

impl NotifyWaiter for SemaphoreNotifyWaiter {
    fn wait(&self, timeout: MicrosDurationU32) -> bool {
        let mut t = timeout.to_millis();
        if t == 0 {
            t = 1;
        }
        self.inner.take(Duration::ms(t)).is_ok()
    }
}

/// `DelayNs` implementation
#[derive(Default)]
pub struct FreeRtosTickDelayNs {}

impl FreeRtosTickDelayNs {
    pub const fn new() -> Self {
        Self {}
    }
}

impl DelayNs for FreeRtosTickDelayNs {
    #[inline]
    fn delay_ns(&mut self, ns: u32) {
        let mut t = FreeRtosTimeoutNs::new().start_ns(ns);
        while !t.timeout() {}
    }

    #[inline]
    fn delay_us(&mut self, us: u32) {
        let mut t = FreeRtosTimeoutNs::new().start_us(us);
        while !t.timeout() {
            CurrentTask::yield_now();
        }
    }

    #[inline]
    fn delay_ms(&mut self, ms: u32) {
        CurrentTask::delay(Duration::ms(ms));
    }
}

#[derive(Default)]
pub struct FreeRtosTimeoutNs {}

impl FreeRtosTimeoutNs {
    pub const fn new() -> Self {
        Self {}
    }
}

impl TimeoutNs for FreeRtosTimeoutNs {
    type TimeoutState = FreeRtosTimeoutState;

    #[inline]
    fn start_ns(&self, timeout: u32) -> Self::TimeoutState {
        Self::TimeoutState::new_ns(timeout)
    }

    #[inline]
    fn start_us(&self, timeout: u32) -> Self::TimeoutState {
        Self::TimeoutState::new_us(timeout)
    }

    #[inline]
    fn start_ms(&self, timeout: u32) -> Self::TimeoutState {
        Self::TimeoutState::new_ms(timeout)
    }
}

#[cfg(cortex_m)]
pub use sys_tick_timeout::*;
#[cfg(cortex_m)]
mod sys_tick_timeout {
    use super::*;
    use core::sync::atomic::{Ordering, compiler_fence};
    use cortex_m::peripheral::SYST;

    #[derive(Clone, Copy)]
    pub struct FreeRtosTimeoutState {
        sys_tick: u32,
        count: u32,
        timeout_tick: u64,
    }

    impl FreeRtosTimeoutState {
        pub fn new_ns(timeout: u32) -> Self {
            let ns = timeout as u64;
            let timeout_tick = (ns * Self::frequency()).div_ceil(1_000_000);
            Self::new(timeout_tick)
        }

        pub fn new_us(timeout: u32) -> Self {
            let us = timeout as u64;
            let timeout_tick = (us * Self::frequency()).div_ceil(1_000);
            Self::new(timeout_tick)
        }

        pub fn new_ms(timeout: u32) -> Self {
            let ms = timeout as u64;
            let timeout_tick = ms * Self::frequency();
            Self::new(timeout_tick)
        }

        #[inline]
        fn frequency() -> u64 {
            // to kHz
            (utils::cpu_clock_hz() / 1000) as u64
        }

        fn new(timeout_tick: u64) -> Self {
            let (sys_tick, count) = Self::now();
            let (sys_tick, count) = Self::add(sys_tick, count, timeout_tick);
            Self {
                sys_tick,
                count,
                timeout_tick,
            }
        }

        fn now() -> (u32, u32) {
            let count = FreeRtosUtils::get_tick_count();
            let sys_tick = SYST::get_current();
            compiler_fence(Ordering::Acquire);
            let count2 = FreeRtosUtils::get_tick_count();
            let sys_tick2 = SYST::get_current();

            if count != count2 {
                (sys_tick2, count2)
            } else {
                (sys_tick, count)
            }
        }

        fn add(sys_tick: u32, count: u32, tick: u64) -> (u32, u32) {
            let reload = (SYST::get_reload() + 1) as u64;
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
    }

    impl TimeoutState for FreeRtosTimeoutState {
        /// Can be reused without calling `restart()`.
        fn timeout(&mut self) -> bool {
            let (sys_tick, count) = Self::now();
            let diff = (count as i32).wrapping_sub(self.count as i32);
            if diff > 1 || (diff == 0 && sys_tick < self.sys_tick) {
                (self.sys_tick, self.count) =
                    Self::add(self.sys_tick, self.count, self.timeout_tick);
                true
            } else {
                false
            }
        }

        fn restart(&mut self) {
            let (sys_tick, count) = Self::now();
            (self.sys_tick, self.count) = Self::add(sys_tick, count, self.timeout_tick);
        }
    }
}
