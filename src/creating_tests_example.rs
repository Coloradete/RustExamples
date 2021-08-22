///applies to code that follows it, in this case its our 'hello()' function
fn hello() -> String{
    "hello".to_string()
}

#[test]
// #[should_panic] //Esta opcion indica que esperamos un resultado erroneo
// #[ignore]        //Esta opcion es para ignorar este test en especifico
fn test_example(){
    assert_eq!("hello", hello());
}