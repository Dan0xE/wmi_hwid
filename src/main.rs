mod lib {
    pub mod query_hwid;
}

fn main() {
    lib::query_hwid::query_hwid();
}
