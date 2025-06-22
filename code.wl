let foo = () => {
    let x: bool | i32 = 5i32;

    let xIsI32 = x::is(i32);

    if xIsI32 && x < 5i32 {
        true
    } else {
        false
    }
};

let res: bool = foo();
