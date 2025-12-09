macro_rules! my_vec {
    ($($e:expr),*) => {
        vec![$($e),*]
    };
}

macro_rules! vec_or_single {
    ($e:expr) =>{
         vec![$e]
    };
    ($($e:expr),+) => {
       vec![$($e),+]
    };
}
macro_rules! with_var {
    ($name:ident,$val:expr,$body:block) => {
       { let  $name = $val;
        $body}
    };
}


fn main() {
    let v = my_vec![1, 2, 3];

    for vi in v {
        println!("{}", vi);
    }
    let a: Vec<i32> = vec_or_single!(10);
    println!("{:?}", a);

     with_var!(x, 10, {
        println!("{}", x + 5);
    });

    // println!("{:?}",v);
}
