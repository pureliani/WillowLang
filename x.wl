type CB = <X>(arg2: X) => X;

let foo = (arg1: CB, val: i32) => {
    arg1(val)
};

let xyz: i32 = foo((woo: i32) => {}, 25i32);
