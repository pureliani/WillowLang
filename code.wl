struct Foo {
    bar: Bar | null
}

struct Bar {
    foo: Foo
}

let x: Foo = Foo {
    bar: Bar {
        foo: Foo {
            bar: null
        }
    }
};
