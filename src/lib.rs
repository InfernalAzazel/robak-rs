//#![feature(allocator_api)]

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[cfg(feature = "jdy")]
pub mod jdy;
pub mod request;
