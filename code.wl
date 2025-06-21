let foo = () => {
    let x: bool | i32 | i64 = 5i32;

    if x::is(i32) && x > 15i32 {
        x = true;
    } else {
        x = 2;
    }
    
    x
};

let res: bool = foo();
