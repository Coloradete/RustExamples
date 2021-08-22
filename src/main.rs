use std::collections::HashMap;

mod Lock;
mod Colors;
mod Unions;
mod Options;
mod Arrays;
mod Slices;
mod Tuples;
mod pattern_matching;
mod generics;
mod Vectors;
mod Hashmap;
mod hashset_test;
mod iterators;
mod strings;
mod number_guessing;
mod functions;
mod structs_and_methods;
mod closures;
mod higher_order_functions;
mod traits_example;
mod into_test;
mod drop_test;
mod operator_overloading;
mod static_dispatch_test;
mod dynamic_dispatch_test;
mod vectors_of_different_types;
mod ownership_test;
mod borrowing_test;
mod lifetime_test;
mod reference_counted_variables_test;
mod atomic_reference_counted_variable_test;
mod circular_references_test;
mod threads_test;
mod creating_tests_example;

fn main()
{
    // Lock::try_lock();
    // Colors::try_color();
    // Unions::call_int_or_float();
    // Options::options_call();
    // Arrays::create_array();
    // Slices::call_slice()
    // Tuples::call_tuples();
    // pattern_matching::do_pattern_matching();
    // Vectors::call_vectors();
    // Hashmap::call_hashmap();
    // hashset_test::call_hashset();
    // iterators::call_iterators();
    // strings::call_strings();
    // strings::call_format();
    // number_guessing::guess_number();
    // functions::call_functions();
    // structs_and_methods::call_structs();
    // closures::call_closures();
    // higher_order_functions::call_higher_order();
    // traits_example::call_traits();
    // traits_example::call_traits_02();
    // into_test::call_into_test();
    // drop_test::call_drop_test();
    // opeartor_overloading::call_operator_overloading();
    // static_dispatch_test::call_static_dispatch_test();
    // dynamic_dispatch_test::call_dynamic_dispatch_test();
    // vectors_of_different_types::call_vectors_of_different_types();
    // ownership_test::call_ownership_test();
    // borrowing_test::call_borrowing_test();
    // lifetime_test::call_lifetime_test();
    // reference_counted_variables_test::reference_counted_variable_call();
    // atomic_reference_counted_variable_test::call_arcv_test();
    // circular_references_test::call_circular_references_test();
    threads_test::call_thread_test();
}
