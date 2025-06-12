struct Foo<J> {
    f: J
}

let foo = <T>(arg: T): Foo<T> => {
    Foo {
        f: arg
    }
};

let x = foo<i32>(1);