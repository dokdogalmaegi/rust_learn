pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

fn main() {
    a::series::of::nested_modules();
}


#[cfg(test)]
mod tests {


    #[test]
    fn it_works() {

    }
}
