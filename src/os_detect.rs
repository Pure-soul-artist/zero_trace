use os_info;

pub fn detect_os() -> String {
    let info = os_info::get();
    format!("{} {}", info.os_type(), info.version())
}