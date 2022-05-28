use sysinfo::{DiskExt, NetworkExt, NetworksExt, ProcessorExt, System, SystemExt};

pub struct Info {
    sys: System,
}

impl Info {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self { sys }
    }

    fn b_to_gb(u: u64) -> f64 {
        u as f64 / 1000000.00
    }

    fn avg(list: Vec<i32>) -> i32 {
        let cnt: i32 = list.len() as i32;
        let mut tmp: i32 = 0;

        for i in list {
            tmp += i;
        }

        tmp / cnt
    }

    pub fn os_info(&self) -> &Self {
        println!("OS:\t\t{} {}", self.os_name(), self.os_version());
        self
    }

    pub fn os_name(&self) -> String {
            format!("{}", self.sys
                .name()
                .unwrap_or("not detected".to_string())
                .replace("\"", ""))
    }

    pub fn os_version(&self) -> String {
        format!("{}", self.sys
                .os_version()
                .unwrap_or("1.0.0".to_string())
                .replace("\"", ""))
    }

    pub fn kernel_version(&self) -> &Self {
        println!("Kernel version:\t{}", self.sys
                .kernel_version()
                .unwrap_or("not detected".to_string())
                .replace("\"", ""));
                self
    }

    pub fn memory(&self) -> &Self {
        println!(
            "Memory:\t\t{:.2} / {:.2} GB",
            Self::b_to_gb(self.sys.used_memory()),
            Self::b_to_gb(self.sys.total_memory())
        );
        self
    }

    pub fn cpu_name(&self) -> &Self {
        println!("CPU Name:\t{}", self.sys.global_processor_info().brand());
        self
    }

    pub fn cpu_avg(&self) -> &Self {
        let mut v: Vec<i32> = vec![];
        for proc in self.sys.processors() {
            v.push(proc.cpu_usage().round() as i32);
        }
        println!("CPU avg.:\t{}%", Self::avg(v));

        self
    }

    pub fn disks(&self) -> &Self {
        print!("Disks:\t\t");
        let disks = self.sys.disks();
        for disk in disks {
            if disk
                == match disks.last() {
                    Some(r) => r,
                    None => {
                        break;
                    }
                }
            {
                print!("{}", disk.name().to_string_lossy());
            } else {
                print!("{}, ", disk.name().to_string_lossy());
            }
        }
        println!();
        self
    }

    pub fn uptime(&self) -> &Self {
        let upt : i32 = self.sys.uptime() as i32;
        println!("Uptime:\t\t{}h:{}m:{}s", upt / 60 / 60, upt / 60 % 60, upt % 60 % 60);
        self
    }

    
}