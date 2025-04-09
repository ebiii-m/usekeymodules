pub mod a{
    pub mod series{
        pub mod of{
            pub fn nested_modules() {}
        }
    }
}

use a::series::of;
// or use a::series::of::nested_modules;

fn main(){
    println!("start");
    of::nested_modules();
    println!("end");
}
