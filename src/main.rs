use num::bigint::BigUint;

use std::ops::Rem;

fn main() {
    let mut init1 = Vec::new();
    let mut init2 = Vec::new();

    init1.push(0);
    init2.push(1);

    let mut num1: BigUint = BigUint::new(init1);
    let mut num2: BigUint = BigUint::new(init2);


    for _i in 0..900000 {

        if checker(&num1) == "" {
            println!("{}", &num1);
        }
        else {
            println!("{}", checker(&num1));
        }
        let numholder: BigUint = num1 + &num2;

        num1 = num2;
        num2 = numholder;
    }



}

fn checker (subject: &BigUint) -> String {
    let three: u32 = 3;
    let five: u32 = 5;

    let mut init1 = Vec::new();

    init1.push(0);

    let zero: BigUint = BigUint::new(init1);


    let mut output = String::from("");

    if subject == &zero {
        return output;
    }
    if subject.rem(three) == zero {
        output.push_str("Vittu ");
    }
    if subject.rem(five) == zero {
        output.push_str("Saatana");
    }
    output
}
