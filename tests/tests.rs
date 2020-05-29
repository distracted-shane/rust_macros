#[macro_use]
extern crate rust_macros;

use std::env;

#[test]
fn one_var() {
    // Set environmental variable
    env::set_var("ONE_TEST_VAR_1", "ABCDEzyxwv");

    // Create struct and read var into it
    collect_env![name=TestA; ONE_TEST_VAR_1];
    let test_env = TestA::read();

    // Clean up
    env::remove_var("ONE_TEST_VAR_1");

    // Verify
    assert_eq!(test_env.ONE_TEST_VAR_1, Some(String::from("ABCDEzyxwv")));
}

#[test]
fn three_var() {
    // Set environmental variables
    env::set_var("TWO_TEST_VAR_1", "AaBbCc123");
    env::set_var("TWO_TEST_VAR_2", "AaBbCc456");
    env::set_var("TWO_TEST_VAR_3", "AaBbCc789");

    // Create struct and read vars into it
    collect_env![name=TestB; TWO_TEST_VAR_1, TWO_TEST_VAR_2, TWO_TEST_VAR_3];
    let test_env = TestB::read();

    // Clean up
    env::remove_var("TWO_TEST_VAR_1");
    env::remove_var("TWO_TEST_VAR_1");
    env::remove_var("TWO_TEST_VAR_1");

    // Verify
    assert_eq!(test_env.TWO_TEST_VAR_1, Some(String::from("AaBbCc123")));
    assert_eq!(test_env.TWO_TEST_VAR_2, Some(String::from("AaBbCc456")));
    assert_eq!(test_env.TWO_TEST_VAR_3, Some(String::from("AaBbCc789")));
}

#[test]
fn three_var_one_missing() {
    // Set environmental variables
    env::set_var("THREE_TEST_VAR_1", "AaBbCc123");
    env::set_var("THREE_TEST_VAR_3", "AaBbCc789");

    // Create struct and read vars into it
    collect_env![name=TestC; THREE_TEST_VAR_1, THREE_TEST_VAR_2, THREE_TEST_VAR_3];
    let test_env = TestC::read();

    // Clean up
    env::remove_var("THREE_TEST_VAR_1");
    env::remove_var("THREE_TEST_VAR_3");

    // Verify
    assert_eq!(test_env.THREE_TEST_VAR_1, Some(String::from("AaBbCc123")));
    assert_eq!(test_env.THREE_TEST_VAR_2, None);
    assert_eq!(test_env.THREE_TEST_VAR_3, Some(String::from("AaBbCc789")));
}

#[test]
fn three_var_all_missing() {
    // Create struct and read vars into it
    collect_env![name=TestD; NULL_TEST_VAR_1, NULL_TEST_VAR_2, NULL_TEST_VAR_3];
    let test_env = TestD::read();

    // Verify
    assert_eq!(test_env.NULL_TEST_VAR_1, None);
    assert_eq!(test_env.NULL_TEST_VAR_2, None);
    assert_eq!(test_env.NULL_TEST_VAR_3, None);
}
