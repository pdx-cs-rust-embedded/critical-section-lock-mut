#![doc(html_root_url = "https://docs.rs/critical-section-lock-mut/0.1.0")]
#![no_std]
//! Lock data with mutable access on a single-core
//! processor.  The locking is provided by a
//! `critical-section::Mutex`.
//!
//! # Example
//!
//! ```text
//! use critical_section_lock_mut::LockMut;
//!
//! static SHARED: LockMut<u8> = LockMut::new();
//!
//! fn main() {
//!     SHARED.init(3);
//!     SHARED.with_lock(|u| *u += 1);
//! ```

use core::cell::RefCell;

pub use critical_section;
use critical_section::Mutex;

/// This datatype provides a lock with interior mutability
/// for the data inside.
#[derive(Debug)]
pub struct LockMut<T>(Mutex<RefCell<Option<T>>>);

impl<T> LockMut<T> {
    /// Create a new empty `LockMut`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use critical_section_lock_mut::LockMut;
    /// static LOCKED_DATA: LockMut<u8> = LockMut::new();
    /// ```
    pub const fn new() -> Self {
        LockMut(Mutex::new(RefCell::new(None)))
    }

    /// Initialize a previously-uninitialized `LockMut`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use critical_section_lock_mut::LockMut;
    /// static LOCKED_DATA: LockMut<u8> = LockMut::new();
    /// LOCKED_DATA.init(5);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if this `LockMut` is to be initialized a second time.
    pub fn init(&self, val: T) {
        critical_section::with(|cs| {
            let mut cell = self.0.borrow(cs).borrow_mut();
            assert!(cell.is_none(), "lock reinitialized");
            *cell = Some(val);
        });
    }

    /// Locks, then runs the closure `f` with a mutable
    /// reference to the locked data.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use critical_section_lock_mut::LockMut;
    /// static LOCKED_DATA: LockMut<u8> = LockMut::new();
    /// LOCKED_DATA.init(5);
    /// LOCKED_DATA.with_lock(|u| *u += 1);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if this `LockMut` is uninitialized.
    pub fn with_lock<F: FnOnce(&mut T)>(&self, f: F) {
        critical_section::with(|cs| {
            // &LockMut<T> → &Mutex<RefCell<Option<T>>> →
            // &RefCell<Option<T>> → &mut Option<T>
            let mut cell = self.0.borrow(cs).borrow_mut();
            // &mut Option<T> → Option<&mut T> → &mut T
            let val_mut = cell.as_mut().expect("empty lock");
            f(val_mut);
        });
    }
}
