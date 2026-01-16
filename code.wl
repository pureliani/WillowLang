from "./code2.wl" {}

fn main(): #Name(string) {
    let userIdentifier: #Id(u32) | #Name(string) = #Name("Gabriel");

    if userIdentifier::is(#Name) { userIdentifier.value } else { "Guest" }
}
