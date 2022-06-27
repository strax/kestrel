// use binaryen::*;
//
// #[test]
// fn test_block_get_set_name() {
//     let mut module = Module::new();
//     let mut block = Block::new(&mut module, "block");
//     assert_eq!(block.name().to_str().unwrap(), "block");
//
//     block.set_name("another one");
//     assert_eq!(block.name().to_str().unwrap(), "another one");
// }
//
// #[test]
// fn test_block_add_remove_child() {
//     let mut module = Module::new();
//     let mut block = Block::new(&module, "block");
//     let nop = Nop::new(&module);
//     block.append_child(&nop);
//     assert_eq!(block.len(), 1);
//     block.remove_child_at(0);
//     assert!(block.is_empty());
// }
//
// #[test]
// fn test_expression_downcast() {
//     let module = Module::new();
//     let x: Block;
//     let nop = Nop::new(&module);
//     let expr = &nop as &dyn Expression;
//     assert!(expr.downcast_ref::<Nop>().unwrap() == &nop);
// }