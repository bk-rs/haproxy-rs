#[cfg(feature = "_integration_tests")]
#[path = "integration_tests"]
mod integration_tests {
    mod helpers;

    #[cfg(test)]
    mod show_env;
    #[cfg(test)]
    mod show_info;
    #[cfg(test)]
    mod show_stat;
}
