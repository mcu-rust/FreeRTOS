use crate::*;
use core::{cell::UnsafeCell, marker::PhantomData};
use os_trait::{prelude::*, FakeRawMutex, MicrosDurationU32, TickInstant, TickTimeoutNs};

/// `OsInterface` implementation, the N can be choose between [`SemaphoreNotifier`]
pub struct FreeRTOS<T, N> {
    _t: PhantomData<T>,
    _n: PhantomData<N>,
}

unsafe impl<T, N> Send for FreeRTOS<T, N> {}
unsafe impl<T, N> Sync for FreeRTOS<T, N> {}

impl<T, N> OsInterface for FreeRTOS<T, N>
where
    T: TickInstant,
    N: NotifyBuilder,
{
    type RawMutex = FakeRawMutex;
    type Notifier = N::Notifier;
    type NotifyWaiter = N::Waiter;
    type Timeout = TickTimeoutNs<T>;

    const O: Self = Self {
        _t: PhantomData,
        _n: PhantomData,
    };

    #[inline]
    fn yield_thread() {
        CurrentTask::yield_now();
    }

    #[inline]
    fn delay() -> impl DelayNs {
        FreeRtosTickDelayNs::<T>::new()
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
        let mut t = TickTimeoutNs::<T>::start_ns(ns);
        while !t.timeout() {}
    }

    #[inline]
    fn delay_us(&mut self, us: u32) {
        let mut t = TickTimeoutNs::<T>::start_us(us);
        while !t.timeout() {
            CurrentTask::yield_now();
        }
    }

    #[inline]
    fn delay_ms(&mut self, ms: u32) {
        CurrentTask::delay(Duration::ms(ms));
    }
}
