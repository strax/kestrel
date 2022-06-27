use binaryen::*;
use cstr::cstr;

#[test]
fn module_add_remove_tag() {
    let mut module = Module::new();
    let tag1 = module.add_tag(cstr!("tag1"), Type::INT32, Type::INT32);
    let tag2 = module.get_tag(cstr!("tag1")).unwrap();
    assert!(tag1 == tag2);
}