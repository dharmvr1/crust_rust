macro_rules! dbg_expr {
    ($e:expr) => {
        println!("{} = {}", stringify!($e), $e);
    };
}

fn main() {
    // dbg_expr!(1 + 2 * 3); /*it can print like 1+2*3=7 */
    dbg_expr!({
    println!("side effect");
    10
});
    
}
