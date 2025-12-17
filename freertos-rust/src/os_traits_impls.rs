use crate::*;
use core::{cell::UnsafeCell, marker::PhantomData};
use os_trait::{
    FakeRawMutex, KilohertzU32, MicrosDurationU32, TickInstant, TickTimeoutNs, TickTimeoutState,
    prelude::*,
};

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
    type Timeout = TickTimeoutNs<FreeRtosTickInstant>;
    type TimeoutState = TickTimeoutState<FreeRtosTickInstant>;
    type Delay = FreeRtosTickDelayNs;

    const O: Self = Self { _n: PhantomData };

    #[inline]
    fn yield_thread() {
        CurrentTask::yield_now();
    }

    #[inline]
    fn timeout() -> Self::Timeout {
        TickTimeoutNs::<FreeRtosTickInstant>::new()
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
        let mut t = TickTimeoutNs::<FreeRtosTickInstant>::new().start_ns(ns);
        while !t.timeout() {}
    }

    #[inline]
    fn delay_us(&mut self, us: u32) {
        let mut t = TickTimeoutNs::<FreeRtosTickInstant>::new().start_us(us);
        while !t.timeout() {
            CurrentTask::yield_now();
        }
    }

    #[inline]
    fn delay_ms(&mut self, ms: u32) {
        CurrentTask::delay(Duration::ms(ms));
    }
}

#[derive(Clone, Copy)]
pub struct FreeRtosTickInstant {
    sys_tick: u32,
    tick_count: u32,
}

#[cfg(cortex_m)]
impl TickInstant for FreeRtosTickInstant {
    #[inline]
    fn frequency() -> KilohertzU32 {
        use os_trait::fugit::HertzU32;
        HertzU32::from_raw(utils::cpu_clock_hz()).convert()
    }

    fn now() -> Self {
        use core::sync::atomic::{Ordering, compiler_fence};
        use cortex_m::peripheral::SYST;

        let tick_count = FreeRtosUtils::get_tick_count();
        let sys_tick = SYST::get_current();
        compiler_fence(Ordering::Acquire);
        let tick_count2 = FreeRtosUtils::get_tick_count();
        let sys_tick2 = SYST::get_current();

        if tick_count != tick_count2 {
            Self {
                sys_tick: sys_tick2,
                tick_count: tick_count2,
            }
        } else {
            Self {
                sys_tick,
                tick_count,
            }
        }
    }

    fn tick_since(self, earlier: Self) -> u32 {
        use cortex_m::peripheral::SYST;
        let reload = SYST::get_reload() + 1;
        (self.tick_count - earlier.tick_count) * reload + earlier.sys_tick - self.sys_tick
    }
}

#[cfg(not(cortex_m))]
impl TickInstant for FreeRtosTickInstant {
    #[inline]
    fn frequency() -> KilohertzU32 {
        todo!()
    }

    fn now() -> Self {
        Self {
            sys_tick: 0,
            tick_count: 0,
        }
    }

    fn tick_since(self, _earlier: Self) -> u32 {
        self.tick_count + self.sys_tick
    }
}
