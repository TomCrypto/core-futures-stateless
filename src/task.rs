use core::ptr::null;

pub use core::task::*;

pub fn stateless_waker() -> Waker {
    unsafe { Waker::from_raw(DUMMY_RAW_WAKER) }
}

const DUMMY_RAW_WAKER: RawWaker = RawWaker::new(null(), DUMMY_VTABLE);

fn dummy_waker_clone(_: *const ()) -> RawWaker {
    DUMMY_RAW_WAKER
}

fn dummy_waker_no_op(_: *const ()) {
    // we cannot do anything here
}

const DUMMY_VTABLE: &RawWakerVTable = &RawWakerVTable::new(
    dummy_waker_clone,
    dummy_waker_no_op,
    dummy_waker_no_op,
    dummy_waker_no_op,
);
