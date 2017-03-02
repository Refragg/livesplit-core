use livesplit_core::component::previous_segment::Component as PreviousSegmentComponent;
use livesplit_core::Timer;
use super::{alloc, drop, acc, output_vec};
use libc::c_char;

pub type OwnedPreviousSegmentComponent = *mut PreviousSegmentComponent;

#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponent_new() -> OwnedPreviousSegmentComponent {
    alloc(PreviousSegmentComponent::new())
}

#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponent_drop(this: OwnedPreviousSegmentComponent) {
    drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponent_state_as_json(this: *const PreviousSegmentComponent,
                                                        timer: *const Timer)
                                                        -> *const c_char {
    output_vec(|o| { acc(this).state(acc(timer)).write_json(o).unwrap(); })
}