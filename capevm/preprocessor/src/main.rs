use opcodes::Opcode;
use classfile_parser::class_parser;


fn main() {
    let classfile_bytes = include_bytes!("../../benchmarks/bsort16/java/capevm/bench/Benchmark.class");

    match class_parser(classfile_bytes) {
        Ok((_, class_file)) => {
            println!(
                "version {},{} \
                 const_pool({}), \
                 this=const[{}], \
                 super=const[{}], \
                 interfaces({}), \
                 fields({}), \
                 methods({}), \
                 attributes({}), \
                 access({:?})",
                class_file.major_version,
                class_file.minor_version,
                class_file.const_pool_size,
                class_file.this_class,
                class_file.super_class,
                class_file.interfaces_count,
                class_file.fields_count,
                class_file.methods_count,
                class_file.attributes_count,
                class_file.access_flags
            );
        }
        Err(_) => panic!("Failed to parse"),
    };
}
