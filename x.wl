struct User<T: i32> {
    bar: T
}

let u: User<i8> = User<i64> {
    bar: 2
};
