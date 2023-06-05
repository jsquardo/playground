enum RSEnum {
    Foo(i32),
    Bar(String),
    Baz(Vec<String>),
}

fn main() {
    let foo = RSEnum::Foo(5);

    if let RSEnum::Foo(value) = foo {}
    match foo {
        RSEnum::Foo(value) => {}
    }
}
