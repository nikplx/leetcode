#![feature(iter_intersperse)]
#![feature(trusted_random_access)]
extern crate core;

mod romain_to_int;
mod add_two_numbers;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
