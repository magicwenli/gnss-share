use libc;
use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;

pub struct Config {
    pub dev_path: String,
    pub port: u16,
    pub net_iface: Option<String>,
    pub no_tcp: bool,
    pub socket_path: Option<String>,
    pub baudrate: u32,
    pub daemonize: bool,
}

impl Config {
    pub fn get_ip(&self) -> String {
        match self.net_iface {
            Some(ref iface) => unsafe { Config::get_ip_for_iface(iface) },

            None => "0.0.0.0".to_string(),
        }
    }

    unsafe fn get_ip_for_iface(iface: &str) -> String {
        let mut addr_ptr = ptr::null_mut();

        let ret = libc::getifaddrs(&mut addr_ptr);
        if ret != 0 || addr_ptr.is_null() {
            return "0.0.0.0".to_string();
        }

        while !addr_ptr.is_null() {
            let addr = *addr_ptr;
            addr_ptr = addr.ifa_next;

            let name = match CStr::from_ptr(addr.ifa_name).to_str() {
                Ok(n) => n,
                Err(e) => {
                    println!("{}", e);

                    continue;
                }
            };

            if name != iface || addr.ifa_addr.is_null() {
                continue;
            }

            let mut host = CString::from_vec_unchecked(vec![0u8; libc::NI_MAXHOST as usize]);
            let size = match i32::from((*addr.ifa_addr).sa_family) {
                libc::AF_INET => mem::size_of::<libc::sockaddr_in>() as u32,
                libc::AF_INET6 => mem::size_of::<libc::sockaddr_in6>() as u32,
                _ => continue,
            };
            let host_ptr = host.into_raw() as *mut libc::c_char;
            let ret = libc::getnameinfo(
                addr.ifa_addr,
                size,
                host_ptr,
                libc::NI_MAXHOST,
                ptr::null_mut(),
                0,
                libc::NI_NUMERICHOST,
            );
            host = CString::from_raw(host_ptr);
            if ret != 0 {
                return "0.0.0.0".to_string();
            }

            match host.into_string() {
                Ok(ip) => return ip,
                Err(e) => {
                    println!("{}", e);

                    continue;
                }
            }
        }

        "0.0.0.0".to_string()
    }
}
