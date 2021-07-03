// This code is mostly based on spin-rs(https://github.com/mvdnes/spin-rs)

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

use super::arch::x86_64;

pub type Mutex<T> = SpinMutex<T>;
pub type MutexGuard<'a, T> = SpinMutexGuard<'a, T>;

pub struct SpinMutex<T: ?Sized> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

pub struct SpinMutexGuard<'a, T: ?Sized + 'a> {
    lock: &'a AtomicBool,
    data: &'a mut T,
}

impl<T> SpinMutex<T> {
    pub const fn new(data: T) -> SpinMutex<T> {
        SpinMutex {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn into_inner(self) -> T {
        let SpinMutex { data, .. } = self;
        data.into_inner()
    }

    pub fn as_mut_ptr(&self) -> *mut T {
        self.data.get()
    }
}

impl<T: ?Sized> SpinMutex<T> {
    pub fn is_locked(&self) -> bool {
        self.lock.load(Ordering::Relaxed)
    }

    pub fn lock(&self) -> SpinMutexGuard<T> {
        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            while self.is_locked() {
                //arch_loop();
                x86_64::nop();
            }
        }

        SpinMutexGuard {
            lock: &self.lock,
            data: unsafe { &mut *self.data.get() },
        }
    }

    pub fn try_lock(&self) -> Option<SpinMutexGuard<T>> {
        if self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(SpinMutexGuard {
                lock: &self.lock,
                data: unsafe { &mut *self.data.get() },
            })
        } else {
            None
        }
    }

    pub unsafe fn force_unlock(&self) {
        self.lock.load(Ordering::Release);
    }
}

impl<'a, T: ?Sized> Deref for SpinMutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.data
    }
}

impl<'a, T: ?Sized> DerefMut for SpinMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.data
    }
}

impl<'a, T: ?Sized> Drop for SpinMutexGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.store(false, Ordering::Release);
    }
}

unsafe impl<T: ?Sized + Send> Send for SpinMutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for SpinMutex<T> {}
