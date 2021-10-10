#[cfg(test)]
mod test_lib_cap {
    use cap_macro::CapMacro;
    use cap_macro_derive::CapMacro;

    #[derive(CapMacro)]
    struct CanDeleteStuff;

    #[test]
    fn does_it_work() {
        CanDeleteStuff::hello();
    }
}
