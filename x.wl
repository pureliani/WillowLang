struct Foo {
    b: Bar
}

struct Bar {
    f: Foo | null
}

let v: Foo = Foo {
    b: Bar {
        f: Foo {
            b: Bar {
                f: null
            }
        }
    }
};
