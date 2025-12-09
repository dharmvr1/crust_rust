macro_rules! enum_with_str {
    ($enum_name:ident{$($attr_name:ident => $name:expr),* ,$(,)?}) => {
        #[derive(Debug)]
        enum  $enum_name{
            $($attr_name),*
          }

        impl $enum_name {
          fn as_str(&self) -> &'static str {
            match self{
                $($enum_name::$attr_name => $name),*
            }
          }



        }

        impl std::str::FromStr for $enum_name {

                type Err =();


          fn from_str(s: &str) -> Result<Self, Self::Err>{
              match s {
                    $($name => Ok($enum_name::$attr_name)),*,
                    _ => Err(())
              }
          }
        }



    };
}

enum_with_str! {
    Color {
        Red   => "red",
        Green => "green",
        Blue  => "blue",
    }
}

fn main() {
    let c = Color::Red;
    println!("{}", c.as_str()); // "red"

    // stretch goal, if you implement FromStr:
    use std::str::FromStr;
    let g = Color::from_str("green").unwrap();
    println!("{:?}", g); // should be Color::Green
}
