use crate::*;
use core::{cell::UnsafeCell, marker::PhantomData};
use os_traits::{timeout::tick::*, *};

/// `OsInterface` implementation, the N can be choose between [`SemaphoreNotifier`]
pub struct FreeRTOS<T, N> {
    _t: PhantomData<T>,
    _n: PhantomData<N>,
}

unsafe impl<T, N> Send for FreeRTOS<T, N> {}
unsafe impl<T, N> Sync for FreeRTOS<T, N> {}

impl<T, N> FreeRTOS<T, N>
where
    T: TickInstant,
    N: NotifyBuilder,
{
    pub fn new() -> Self {
        Self {
            _t: PhantomData,
            _n: PhantomData,
        }
    }
}

impl<T, N> OsInterface for FreeRTOS<T, N>
where
    T: TickInstant,
    N: NotifyBuilder,
{
    type RawMutex = FakeRawMutex;
    type NotifyBuilder = N;

    #[inline]
    fn yield_thread() {
        CurrentTask::yield_now();
    }

    #[inline]
    fn timeout() -> impl TimeoutNs {
        TickTimeoutNs::<T>::new()
    }

    #[inline]
    fn delay() -> impl DelayNs {
        FreeRtosTickDelayNs::<T>::new()
    }
}

/// `NotifyBuilder` implementation for task notification
pub struct TaskNotifier {
    inner: Arc<UnsafeCell<Task>>,
}

unsafe impl Send for TaskNotifier {}
unsafe impl Sync for TaskNotifier {}

impl TaskNotifier {
    pub fn new() -> (Self, Self) {
        let task = unsafe { Task::from_raw_handle(core::ptr::null()) };
        let inner = Arc::new(UnsafeCell::new(task));
        let inner2 = Arc::clone(&inner);
        (Self { inner }, Self { inner: inner2 })
    }
}

impl NotifyBuilder for TaskNotifier {
    fn build() -> (impl Notifier, impl NotifyWaiter) {
        Self::new()
    }
    fn build_isr() -> (impl NotifierIsr, impl NotifyWaiter) {
        Self::new()
    }
}

impl Notifier for TaskNotifier {
    fn notify(&self) -> bool {
        let inner = unsafe { &*self.inner.get() };
        if inner.raw_handle() == core::ptr::null() {
            false
        } else {
            inner.set_notification_value(1);
            true
        }
    }
}

impl NotifierIsr for TaskNotifier {
    /// Should be called at the end of interrupt
    fn notify_from_isr(&self) -> bool {
        let inner = unsafe { &*self.inner.get() };
        if inner.raw_handle() == core::ptr::null() {
            false
        } else {
            let mut ctx = InterruptContext::new();
            inner
                .notify_from_isr(&mut ctx, TaskNotification::OverwriteValue(1))
                .is_ok()
        }
    }
}

impl NotifyWaiter for TaskNotifier {
    fn wait(&self, timeout: MicrosDurationU32) -> bool {
        let inner = unsafe { &mut *self.inner.get() };
        if inner.raw_handle() == core::ptr::null() {
            if let Ok(task) = Task::current() {
                critical_section::with(|_| {
                    if inner.raw_handle() == core::ptr::null() {
                        *inner = task;
                    }
                });
                if let Ok(val) =
                    inner.wait_for_notification(0, u32::MAX, Duration::ms(timeout.to_millis()))
                {
                    return val != 0;
                }
            }
        } else if let Ok(val) =
            inner.wait_for_notification(0, u32::MAX, Duration::ms(timeout.to_millis()))
        {
            return val != 0;
        }
        false
    }
}

/// `NotifyBuilder` implementation for [`Semaphore`]
pub struct SemaphoreNotifier {
    inner: Arc<Semaphore>,
}

impl SemaphoreNotifier {
    pub fn new() -> (Self, Self) {
        let inner = Arc::new(Semaphore::new_binary().unwrap());
        let inner2 = Arc::clone(&inner);
        (Self { inner }, Self { inner: inner2 })
    }
}

impl NotifyBuilder for SemaphoreNotifier {
    fn build() -> (impl Notifier, impl NotifyWaiter) {
        Self::new()
    }
    fn build_isr() -> (impl NotifierIsr, impl NotifyWaiter) {
        Self::new()
    }
}

impl Notifier for SemaphoreNotifier {
    fn notify(&self) -> bool {
        self.inner.give()
    }
}

impl NotifierIsr for SemaphoreNotifier {
    /// Should be called at the end of interrupt
    fn notify_from_isr(&self) -> bool {
        let mut ctx = InterruptContext::new();
        self.inner.give_from_isr(&mut ctx)
    }
}

impl NotifyWaiter for SemaphoreNotifier {
    fn wait(&self, timeout: MicrosDurationU32) -> bool {
        self.inner.take(Duration::ms(timeout.to_millis())).is_ok()
    }
}

/// [`DelayNs`](`embedded_hal::delay::DelayNs`) implementation
pub struct FreeRtosTickDelayNs<T> {
    _t: PhantomData<T>,
}

impl<T> FreeRtosTickDelayNs<T>
where
    T: TickInstant,
{
    pub fn new() -> Self {
        Self { _t: PhantomData }
    }
}

impl<T> DelayNs for FreeRtosTickDelayNs<T>
where
    T: TickInstant,
{
    #[inline]
    fn delay_ns(&mut self, ns: u32) {
        let t = TickTimeoutNs::<T>::new();
        let mut ts = t.start_ns(ns);
        while !ts.timeout() {}
    }

    #[inline]
    fn delay_us(&mut self, us: u32) {
        let t = TickTimeoutNs::<T>::new();
        let mut ts = t.start_us(us);
        while !ts.timeout() {
            CurrentTask::yield_now();
        }
    }

    #[inline]
    fn delay_ms(&mut self, ms: u32) {
        CurrentTask::delay(Duration::ms(ms));
    }
}
