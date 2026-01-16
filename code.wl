type User = {
    id: #U32(u32) | #UUID(string),
};

fn main(): u32 {
    let u: User = { id: #U32(17u32) };

    u.id.value
}
