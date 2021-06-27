pub mod main {
    use core::{cell::{RefCell, Ref, RefMut}, marker::PhantomData};

    pub trait GlobalData<T> {
        fn set(&mut self, value: T);
    }

    #[derive(Clone, Copy)]
    pub struct MainThreadMarker {
        _marker: PhantomData<*const ()>,
    }

    impl MainThreadMarker {
        pub unsafe fn new() -> Self {
            Self {
                _marker: PhantomData,
            }
        }
    }

    pub struct MainThreadRefCell<T>(RefCell<T>);

    unsafe impl<T> Send for MainThreadRefCell<T> {}
    unsafe impl<T> Sync for MainThreadRefCell<T> {}

    impl<T: GlobalData<T>> MainThreadRefCell<T> {
        pub const fn new(value: T) -> Self {
            Self(RefCell::new(value))
        }

        pub fn set(&self, _marker: MainThreadMarker, value: T) {
            let borrow = self.0.borrow_mut();
            RefMut::map(borrow, |b| {
                &mut b.set(value);
                b
            });
        }

        pub fn borrow_ref(&self, _marker: MainThreadMarker) -> Ref<T> {
            self.0.borrow()
        }

        pub fn borrow_refmut(&self, _marker: MainThreadMarker) -> RefMut<T> {
            self.0.borrow_mut()
        }
    }
}
