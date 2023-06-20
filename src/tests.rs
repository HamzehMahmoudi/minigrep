
use minigrep::{search, search_case_insensetive};

#[test]
fn test_case_sensetive(){
    let appearance = search("test", "Tested\n not Tested");
    assert!(appearance.is_empty())
}
#[test]
fn test_case_insensetive(){
    let appearance = search_case_insensetive("test", "Tested\nnot Tested");
    assert_eq!(appearance, vec![(1, "Tested"), (2,"not Tested")])
}

