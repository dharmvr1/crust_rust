// macro_rules! make_struct {
//     ($name:ident {$($atrr:ident:$type:ty),* $(,)? }) => {
//         struct $name{
//             $($atrr:$type),*
//         }

//     };
// }

macro_rules! make_struct_with_new {
    ($name:ident{$($field_name:ident:$field_type:ty),* $(,)?}) => {
            struct $name {
                $($field_name:$field_type),*
            }

            impl $name{
                fn new($($field_name:$field_type),*) -> Self {
                    $name {
                           $($field_name),*
                    }
                }
            }
    };
}

// make_struct!(User {
//     id: u64,
//     name: String,
//     active: bool,
// });

make_struct_with_new!(User {
    id: u64,
    name: String,
    active: bool,
});
fn main() {
    let u = User {
        id: 1,
        name: "abc".to_string(),
        active: true,
    };

    // println!("id = {}, name = {}, active = {}", u.id, u.name, u.active);

    let _u = User::new(1, "abc".to_string(), true);
    println!("id = {}, name = {}, active = {}", u.id, u.name, u.active);
}
