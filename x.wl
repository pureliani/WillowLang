struct User<T: bool | i32, Y: bool> {
    a: T,
    b: Y
}

let u = User {
    a: 1i32,
    b: 23
};
