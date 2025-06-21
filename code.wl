let foo = () => {
    let x: bool | i32 = 5i32;

    if x::is(bool) && x > 15i32 {
        x = true;
    } else {
        x = false;
    }

    x
};

let res: bool = foo();
