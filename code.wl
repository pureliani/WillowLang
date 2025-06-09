

type CB = <X>(arg: X) => X;

let foo = (cb: CB) => {
    cb(5)
};

let x = foo((arg: i32) => {
    arg
});
