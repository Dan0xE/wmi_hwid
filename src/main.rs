mod lib {
    pub mod query_hwid;
}

mod structs {
    pub mod wmi_structs;
}

fn main() {
    lib::query_hwid::query_hwid("yourkey").unwrap();
}
