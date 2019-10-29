#![allow(unused_mut)]

#[cfg(feature = "ok")]
// ok
fn main() {
    let berry_instances = vec!["Blackberry", "Strawberry"];

    let ref first_share_ref = &berry_instances; // immutable reference
    let ref second_share_ref = &berry_instances; // immutable reference

    println!("{:?} {:?}", first_share_ref, second_share_ref);
}

#[cfg(feature = "err")]
// error[E0502]
fn main() {
    let mut berry_instances = vec!["Blackberry", "Strawberry"];

    let ref first_share_immut = &berry_instances; // immutable reference
    let ref second_share_mut = &mut berry_instances; // mutable reference

    println!("{:?} {:?}", first_share_immut, second_share_mut);
}

#[cfg(all(not(feature = "ok"), not(feature = "err")))]
fn main() {
    use aide::*;
    hello();
}

// error[E0499]
/*
fn main() {
    let mut berry_instances = vec!["Blackberry", "Strawberry"];

    let first_share_mut = &mut berry_instances; // mutable reference
    let second_share_mut = &mut berry_instances; // mutable reference

    println!("{:?} {:?}", first_share_mut, second_share_mut);
}
*/

// error[E0502]
/*
fn main() {
    let mut berry_instances = vec!["Blackberry", "Strawberry"];

    let first_share_mut = &mut berry_instances; // mutable reference
    let second_share_immut = &berry_instances; // immutable reference

    println!("{:?} {:?}", first_share_mut, second_share_immut);
}
*/
