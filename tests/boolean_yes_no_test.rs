context_type::boolean::yes_no! {
    enum Hello;
}

#[test]
fn yes_no_methods() {
    assert!(Hello::Yes.is_yes());
    assert!(!Hello::Yes.is_no());
    assert!(!Hello::No.is_yes());
    assert!(Hello::No.is_no());

    assert_eq!(Hello::default(), bool::default().into());
    assert_eq!(bool::default(), Hello::default().into());
}
