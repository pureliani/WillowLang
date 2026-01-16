type User = {
    id: #U32(u32) | #UUID(string),
};

fn main(): string {
    let u: User = { id: #U32(17u32) };

    if 1 > 2 {
        u.id = #UUID("hello");
    } else {
        u.id = #UUID("world");
    }

    u.id.value
}
