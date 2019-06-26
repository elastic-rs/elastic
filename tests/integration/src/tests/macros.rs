macro_rules! test_groups {
    ($($group:ident),*) => {
        $(
            mod $group;
        )*

        pub fn all() -> Vec<(&'static str, crate::run_tests::Test)> {
            let mut all = Vec::new();

            $(
                all.extend(self::$group::all());
            )*

            all
        }
    };
}

macro_rules! test_cases {
    ($($case:ident),*) => {
        $(
            mod $case;
        )*

        pub fn all() -> Vec<(&'static str, crate::run_tests::Test)> {
            vec![
                $(
                    (<self::$case::Test as crate::run_tests::IntegrationTest>::path, Box::new(|client| crate::run_tests::test(client, self::$case::Test)))
                ),*
            ]
        }
    };
}

macro_rules! test {
    ($($test:tt)*) => {
        #[derive(Clone, Copy)]
        pub struct Test;

        impl crate::run_tests::IntegrationTest for Test {
            const path: &'static str = module_path!();

            $($test)*
        }
    };
}
