use std::cell::Cell;
use std::ops::Deref;
use std::ptr::NonNull;

struct MyRcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

struct MyRc<T> {
    inner: NonNull<MyRcInner<T>>,
}

impl<T> MyRc<T> {
    fn new(x: T) -> Self {
        let tmp_inner = MyRcInner {
            value: x,
            refcount: Cell::new(1),
        };
        MyRc {
            inner: unsafe {
                NonNull::new_unchecked(Box::into_raw(Box::new(tmp_inner)))
            }
        }
    }

    fn count(&self) -> usize {
        unsafe {
            (self.inner.as_ref()).refcount.get()
        }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        unsafe {
            (self.inner.as_ref()).refcount.set((self.inner.as_ref()).refcount.get() + 1);
        }
        MyRc { inner: self.inner }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        let tmp_inner = unsafe { self.inner.as_ref() };
        let c = tmp_inner.refcount.get();
        if c == 1 {
            let _ = unsafe {
                Box::from_raw(self.inner.as_ptr())
            };
        } else {
            tmp_inner.refcount.set(c - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc() {
        let x = MyRc::new(42);
        let y = x.clone();
        assert_eq!(*x, 42);
        assert_eq!(*y, 42);
        assert_eq!(x.count(), 2);
        assert_eq!(y.count(), 2);
        drop(x);
        assert_eq!(y.count(), 1);
    }
}