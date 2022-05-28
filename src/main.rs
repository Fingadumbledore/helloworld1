use crate::fetch::Info;
mod fetch;

fn main() {
    Info::new().os_info().kernel_version().cpu_name().cpu_avg().memory().disks().uptime();
}
