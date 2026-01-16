fn main(): u32 {
    let userIdentifier: #Id(u32) | #Name(string) = #Name("Gabriel");

    if userIdentifier::is(#Name) { 
        1u32
    } else { 
        userIdentifier.value
    }
}