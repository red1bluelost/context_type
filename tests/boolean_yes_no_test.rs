#[test]
fn yes_no_methods() {
    context_type::boolean::yes_no! {
        enum Hello;
    }

    assert!(Hello::Yes.is_yes());
    assert!(!Hello::Yes.is_no());
    assert!(!Hello::No.is_yes());
    assert!(Hello::No.is_no());

    assert_eq!(true, Hello::Yes.into());
    assert_eq!(Hello::Yes, true.into());
    assert_eq!(false, Hello::No.into());
    assert_eq!(Hello::No, false.into());
}

#[test]
fn yes_no_default() {
    context_type::boolean::yes_no! {
        enum DefTrue {
            default = true
        }
    }
    assert_eq!(DefTrue::default(), DefTrue::Yes);
    assert_eq!(true, DefTrue::default().into());

    context_type::boolean::yes_no! {
        enum DefFalse {
            default = false,
        }
    }
    assert_eq!(DefFalse::default(), DefFalse::No);
    assert_eq!(false, DefFalse::default().into());

    context_type::boolean::yes_no! {
        enum DefYes {
            default = Self::Yes,
        }
    }
    assert_eq!(DefYes::default(), DefYes::Yes);

    context_type::boolean::yes_no! {
        enum DefNo {
            default = DefNo::No,
        }
    }
    assert_eq!(DefNo::default(), DefNo::No);

    context_type::boolean::yes_no! {
        enum DefBool {
            default = bool::default(),
        }
    }
    assert_eq!(DefBool::default(), bool::default().into());
    assert_eq!(bool::default(), DefBool::default().into());

    context_type::boolean::yes_no! {
        enum DefWhy {
            default = {
                let i = 3;
                let c = 4;
                i < c
            }
        }
    }
    assert_eq!(DefWhy::default(), true.into());
}
