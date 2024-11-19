use integ_test_example;

#[test]
fn process_test1() {
    assert_ne!(integ_test_example::get_process_id(), 0, "Error in code");
}

#[test]
fn process_test2() {
    assert_ne!(integ_test_example::get_process_id(), 0, "Error in code");
}
