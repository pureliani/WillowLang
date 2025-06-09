


let foo = (cb: (arg: i32) => i32) => {};

let x = foo((arg: i32) => {
    if 1 < 2 {
        "hello"
    } else if 8 > 9 {
        true
    } else {
        5
    }
});
