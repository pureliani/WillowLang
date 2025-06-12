
let x: <T: i32 | i64>(arg: T) => i32 = <Y: i32 | null>(arg: Y): i32 => {
    arg
};
