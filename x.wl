struct User<T: bool> {
    id: T,
    isAdmin: bool
}

let u = User {
    id: 1,
    isAdmin: true
};
