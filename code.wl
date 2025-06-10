struct Foo<T> {
    f: T
}

let x: Foo = Foo { f: 15 };
let y: Foo<i64> = x;
let z = y;

let t: Foo<i64> = z;