#[macro_export]
macro_rules! dbg_file {
    ($e:expr, $write_to:expr) => {
        match $e {
            p => ::std::fs::write(
                ::std::path::Path::new(&$write_to.to_string()),
                ::std::format!(
                    "[{}:{}:{}] {} = {:#?}",
                    ::std::file!(),
                    ::std::line!(),
                    ::std::column!(),
                    ::std::stringify!($e),
                    p
                ),
            )
            .unwrap(),
        }
    };
}
