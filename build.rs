use std::path::Path;
use vcpkg::Error;

fn main() {



    match vcpkg::find_package("freetype") {
        Ok(freetype) => println!("{:?}", freetype.include_paths) ,
        Err(err) => println!("{}", err),
    }
}
