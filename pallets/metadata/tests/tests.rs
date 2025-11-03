use crate::{mock::*, Error};
use frame_support::{assert_ok, assert_noop};

#[test]
fn store_metadata_works() {
new_test_ext().execute_with(|| {
assert_ok!(MetadataModule::store_metadata(RuntimeOrigin::root(), "key".into(), "value".into()));
assert_eq!(MetadataModule::metadata("key".into()), Some("value".into()));
});
}

#[test]
fn store_metadata_fails_without_root() {
new_test_ext().execute_with(|| {
let user = 1;
assert_noop!(
MetadataModule::store_metadata(RuntimeOrigin::signed(user), "k".into(), "v".into()),
Error::<Test>::NotAuthorized
);
});
}
