use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

pub struct Mutex<T: ?Sized> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

pub struct MutexGuard<'a, T: ?Sized + 'a> {
    lock: &'a AtomicBool,
    data: &'a mut T,
}

impl<T> Mutex<T> {
    #[cfg(feature = "const_fn")]
    pub const fn new(data: T) -> Mutex<T> {
        Mutex {
            lock: ATOMIC_BOOL_INIT,
            data: UnsafeCell::new(data),
        }
    }

    #[cfg(feature = "const_fn")]
    pub fn new(data: T) -> Mutex<T> {
        Mutex {
            lock: ATOMIC_BOOL_INIT,
            data: UnsafeCell::new(data),
        }
    }

    pub fn into_inner(self) -> T {
        let Mutex { data, .. } = self;
        unsafe { data.into_inner() }
    }
}

impl<T: ?Sized> Mutex<T> {
    fn obtain_lock(&self) {
        while self.lock.compare_and_swap(false, true, Ordering::Aquire) != false {
            while self.lock.load(Ordering::Relaxed) {
                
            }
        }
    }
}

