struct Foo<J> {
    f: J
}

let foo = <T>(arg: T): T => {
    Foo {
        f: arg
    }
};

let x: Foo<i32> = foo(1);