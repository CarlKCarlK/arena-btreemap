//! Tests for DormantMutRef reborrow safety pattern.

use super::DormantMutRef;

#[test]
fn dormant_mut_ref_basic() {
    let mut value = 42;
    let (ref1, dormant) = DormantMutRef::new(&mut value);
    *ref1 = 10;
    let ref2 = unsafe { dormant.awaken() };
    *ref2 = 20;
    assert_eq!(value, 20);
}
