use binaryen::*;
use cstr::cstr;

fn main() {
    let module = Module::new();
    module.set_features(Features::all());
    module.auto_drop();
    let square = module.add_function(&FunctionDescriptor {
        name: cstr!("square"),
        params: Type::INT32,
        results: Type::INT32,
        body: Binary::new(
            &module,
            BinaryOp::MulInt32,
            LocalGet::new(&module, 0, Type::INT32),
            LocalGet::new(&module, 0, Type::INT32)
        ),
        vars: &[],
    });
    module.add_function_export(square.name(), square.name());
    module.optimize();
    module.print();
}