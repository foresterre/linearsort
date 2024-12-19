// Too sleepy for this 😴.

macro_rules! i_cant_wait_forever {
    () => {{
        if let Err(e) =
            $crate::set_linear_sort_sleep_duration(std::time::Duration::from_millis(100))
        {
            eprintln!("Failed to set sleep duration: {:?}, ignoring... ", e);
        }
    }};
}

mod sort;
mod sort_by;
mod sort_by_cached_key;
mod sort_by_key;
mod sort_unstable;
mod sort_unstable_by;
mod sort_unstable_by_key;
