struct Foo<T> {
    f: T
}


let func = (): Foo => {
    Foo {
        f: 15
    }
};

let x: Foo<i32> = func();
