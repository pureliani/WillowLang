struct Foo<J> {
    f: J
}

struct Bar<T> {
    b: T
}


let x: Foo<Bar<i32>> = null;