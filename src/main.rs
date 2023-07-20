fn main() {
    borrow_check1();
    borrow_check2();
    // borrow_check3();
    // borrow_check4();
    borrow_check5();
}

fn borrow_check1() {
    let a = 10; // immutable object
    let aref1 = &a; // reference
    let aref2 = &a; // reference
    println!("{}, {}, {}", a, aref1, aref2); // borrow check -> OK
}

fn borrow_check2() {
    let mut a = 10; // mutable object
    let a_ref1 = &a; // reference
    let a_mut_ref1 = &mut a; // mutable reference
    let a_mut_ref2 = &mut a; // mutable reference
    *a_mut_ref2 = 20; // assign
    println!("{}", a); // borrow check -> OK
}

// fn borrow_check3() {
//     let mut a = 10;                  // mutable object
//     let a_ref1 = &a;                // reference
//     let a_mut_ref1 = &mut a;    // mutable reference
//     let a_mut_ref2 = &mut a;    // この時点で a_ref1, a_mut_ref1 は存在しない
//     *a_mut_ref1 = 20;                     // assign (error)
//     println!("{}", a);                    // borrow check -> Error
// }

// fn borrow_check4() {
//     let mut a = 10;                  // mutable object
//     let a_ref1 = &a;                // reference
//     let a_mut_ref1 = &mut a;    // mutable reference
//     let a_mut_ref2 = &mut a;    // この時点で a_ref1, a_mut_ref1 は存在しない
//     println!("{}", a_ref1);                    // borrow check -> Error
// }

fn borrow_check5() {
    let mut a = 10;                 // mutable object
    let a_ref1 = &a;               // reference
    let a_mut_ref1 = &mut a;   // mutable reference
    let a_mut_ref2 = &mut a;   // この時点で a_ref1, a_mut_ref1 は存在しない
    let a_ref2 = &a;               // この時点で a_mut_ref2 は存在しない
    //println!("{}", a_mut_ref2);        // borrow check -> Error
    //println!("{} {}", a_ref1, a_ref2); // borrow check -> Error
    println!("{}", a_ref2);              // borrow check -> OK
}
