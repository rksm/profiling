use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

#[allow(unused)]
#[cfg(not(any(feature = "profile-with-tracy")))]
type Client = ();

#[cfg(feature = "profile-with-tracy")]
type Client = tracy_client::Client;

#[allow(unused)]
static INSTANCE: Lazy<Arc<Mutex<Option<Client>>>> = Lazy::new(Default::default);

pub fn global_profiler_init() {
    #[cfg(feature = "profile-with-optick")]
    {
        optick::start_capture();
        *INSTANCE.lock().unwrap() = Some(());
    }
    #[cfg(feature = "profile-with-tracy")]
    {
        let client = tracy_client::Client::start();
        *INSTANCE.lock().unwrap() = Some(client);
    }
}

#[allow(unused_variables)]
pub fn global_profiler_stop(capture_path: &str) {
    #[cfg(feature = "profile-with-optick")]
    if INSTANCE.lock().unwrap().take().is_some() {
        optick::stop_capture(capture_path);
    }
    #[cfg(feature = "profile-with-tracy")]
    if let Some(client) = INSTANCE.lock().unwrap().take() {
        drop(client);
    }
}

#[allow(unused_variables)]
pub fn set_thread_name(name: &str) {
    #[cfg(feature = "profile-with-optick")]
    optick::register_thread(name);

    #[cfg(feature = "profile-with-superluminal")]
    superluminal_perf::set_current_thread_name(name);

    #[cfg(feature = "profile-with-tracy")]
    if let Some(client) = &*INSTANCE.lock().unwrap() {
        client.set_thread_name(name);
    }
}
