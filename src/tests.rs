use super::*;
use regex::Regex;

#[test]
fn default_xpg_macro_returns_four_words() {
    let re = Regex::new(r"^([A-Z][a-z]+){4}$").unwrap();
    let result = xpg!();
    assert!(re.is_match(&result));
}

#[test]
fn xpg_macro_can_return_three_words() {
    let re = Regex::new(r"^([A-Z][a-z]+){3}$").unwrap();
    let result = xpg!(3);
    assert!(re.is_match(&result));
}

#[test]
fn xpg_macro_can_return_four_words() {
    let re = Regex::new(r"^([A-Z][a-z]+){4}$").unwrap();
    let result = xpg!(4);
    assert!(re.is_match(&result));
}

#[test]
fn xpg_macro_can_return_five_words() {
    let re = Regex::new(r"^([A-Z][a-z]+){5}$").unwrap();
    let result = xpg!(5);
    assert!(re.is_match(&result));
}

#[test]
#[should_panic]
fn xpg_macro_cannot_return_zero_words() {
    let _result = xpg!(0);
}

#[test]
fn xpg_macro_can_return_one_word() {
    let re = Regex::new(r"^([A-Z][a-z]+)$").unwrap();
    let result = xpg!(1);
    assert!(re.is_match(&result));
}

#[test]
fn xpg_can_return_three_words() {
    let re = Regex::new(r"^([A-Z][a-z]+){3}$").unwrap();
    let result = xpg(3);
    assert!(re.is_match(&result));
}

#[test]
fn xpg_can_return_four_words() {
    let re = Regex::new(r"^([A-Z][a-z]+){4}$").unwrap();
    let result = xpg(4);
    assert!(re.is_match(&result));
}

#[test]
fn xpg_can_return_five_words() {
    let re = Regex::new(r"^([A-Z][a-z]+){5}$").unwrap();
    let result = xpg(5);
    assert!(re.is_match(&result));
}

#[test]
#[should_panic]
fn xpg_cannot_return_zero_words() {
    let _result = xpg(0);
}

#[test]
fn xpg_can_return_one_word() {
    let re = Regex::new(r"^([A-Z][a-z]+)$").unwrap();
    let result = xpg(1);
    assert!(re.is_match(&result));
}
