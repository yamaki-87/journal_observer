#[macro_export]
macro_rules! log_if_err {
    ($expr:expr) => {{
        if let Err(e) = $expr {
            log::error!("{}", e);
        }
    }};
}

#[macro_export]
macro_rules! log_if_err_nest {
    ($expr:expr) => {{
        match $expr {
            Ok(inner) => match inner {
                Ok(_) => {
                    log::debug!("errorなし");
                }
                Err(e) => log::error!("{}", e),
            },
            Err(e) => {
                log::error!("{}", e);
            }
        }
    }};
}
