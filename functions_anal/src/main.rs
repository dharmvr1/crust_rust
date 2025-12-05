use std::mem;

fn main() {
    let x = "Здравствуйте";
    let s = "aé日";
    println!("{:?}",s.bytes());
     // bytes: [0x61, 0xc3,0xa9, 0xe6,0x97,0xa5]
    let slice = &s[..3]; // ok: covers 'a' + bytes of 'é'
    println!("{}", slice);
    println!("{:?}", x.chars());
}

//  FnOnce -> self
//  Fn -> &self
//  FnMut -> &mut self
/*
MOve


*/

fn bar<T>() {}

fn quox<F>(f: &mut F)
where
    F: Fn(),
{
    f();
}

fn make_fn() {
    let z = String::from("helo");

    let t = || {
        println!("{}", z);
        println!("{}", String::from("hello"));
    };
    drop(z);
}
