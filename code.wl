fn main(): #Name(string) {
    let userIdentifier: #Id(u32) | #Name(string) = #Name("Gabriel");

    // Return the tag, not the inner string 
    if userIdentifier::is(#Name) { 
        userIdentifier 
    } else { 
        #Name("Guest") 
    }
}