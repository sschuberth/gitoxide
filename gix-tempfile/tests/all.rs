type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

mod registry;
mod tempfile;
