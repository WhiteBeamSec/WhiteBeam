use std::str::FromStr;

build_action! { VerifyCanConnect (par_prog, src_prog, hook, arg_position, args, _act_args, do_return, return_value) {
        // TODO: Depending on LogSeverity, log all use of this action
        let library: &str = &hook.library;
        let library_basename: &str = library.rsplit('/').next().unwrap_or(library);
        let symbol: &str = &hook.symbol;
        if !((/*symbol == "bind" ||*/ symbol == "connect" /*|| symbol.contains("accept")*/) && (library_basename == "libc.so.6")) {
            // TODO: Library path in error inconsistent with rest of application
            unimplemented!("WhiteBeam: The '{}' symbol (from {}) is not supported by the VerifyCanConnect action", symbol, library_basename);
        }
        let any = String::from("ANY");
        let connect_class = String::from("Network/Range/Connect");
        let argument_index = arg_position.expect("WhiteBeam: Lost track of environment") as usize;
        let argument: crate::common::db::ArgumentRow = args[argument_index].clone();
        let argument_sockaddr = argument.real as *const libc::sockaddr;
        let (dest_ip, dest_port) = match unsafe { *argument_sockaddr }.sa_family as i32 {
            libc::AF_INET => {
                let sockaddr_in = argument_sockaddr as *const libc::sockaddr_in;
                let dest_ip = unsafe { std::net::IpAddr::from_str(&crate::common::convert::ipv4_to_string((*sockaddr_in).sin_addr.s_addr)).expect("WhiteBeam: Lost track of environment") };
                let dest_port = unsafe { u16::from_be((*sockaddr_in).sin_port) };
                (dest_ip, dest_port)
            },
            libc::AF_INET6 => {
                let sockaddr_in6 = argument_sockaddr as *const libc::sockaddr_in6;
                let dest_ip = unsafe { std::net::IpAddr::from_str(&crate::common::convert::ipv6_to_string((*sockaddr_in6).sin6_addr.s6_addr)).expect("WhiteBeam: Lost track of environment") };
                let dest_port = unsafe { u16::from_be((*sockaddr_in6).sin6_port) };
                (dest_ip, dest_port)
            },
            _ => {
                /*
                do_return = true;
                return_value = -1;
                platform::set_errno(libc::EAFNOSUPPORT);
                */
                return (hook, args, do_return, return_value);
            }
        };
        let all_allowed_connect_ranges_string: Vec<String> = {
            let whitelist_cache_lock = crate::common::db::WL_CACHE.lock().expect("WhiteBeam: Failed to lock mutex");
            whitelist_cache_lock.iter().filter(|whitelist| (whitelist.class == connect_class) && ((whitelist.parent == par_prog) || (whitelist.parent == any)) && ((whitelist.path == src_prog) || (whitelist.path == any))).map(|whitelist| whitelist.value.clone()).collect()
        };
        // Permit ANY
        if all_allowed_connect_ranges_string.iter().any(|connect_range| connect_range == &any) {
            return (hook, args, do_return, return_value);
        }
        // TODO: Warn on ignored rows (e.g. no port or port range provided, invalid port range)
        // TODO: Refactor
        let matching_connect_ranges: Vec<(String, String)> = all_allowed_connect_ranges_string.iter()
            .map(|whitelist| whitelist.rsplit_once(':'))
            .filter_map(|whitelist_tuple| {
                if let Some((cidr_range, port)) = whitelist_tuple {
                    if let Ok(net) = ipnet::IpNet::from_str(cidr_range) {
                        if let Some(port_range) = port.split_once('-') {
                            if let Ok(port_start) = u16::from_str(port_range.0) {
                                if let Ok(port_end) = u16::from_str(port_range.1) {
                                    if net.contains(&dest_ip) && (port_start <= dest_port) && (dest_port <= port_end) {
                                        return Some((cidr_range.to_string(), port.to_string()));
                                    }
                                }
                            }
                        }
                        if let Ok(port) = u16::from_str(port) {
                            if net.contains(&dest_ip) && (port == dest_port) {
                                return Some((cidr_range.to_string(), port.to_string()));
                            }
                        }
                    }
                }
                None
            })
            .collect();
        if matching_connect_ranges.len() > 0 {
            return (hook, args, do_return, return_value);
        }
        if !(crate::common::db::get_prevention()) {
            crate::common::event::send_log_event(libc::LOG_NOTICE, format!("Detection: {} -> {} connected to {}:{} (VerifyCanConnect)", &par_prog, &src_prog, &dest_ip, &dest_port));
            return (hook, args, do_return, return_value);
        }
        // Permit authorized connections
        if crate::common::db::get_valid_auth_env() {
            return (hook, args, do_return, return_value);
        }
        // Deny by default
        event::send_log_event(libc::LOG_WARNING, format!("Prevention: Blocked {} -> {} from connecting to {}:{} (VerifyCanConnect)", &par_prog, &src_prog, &dest_ip, &dest_port));
        do_return = true;
        return_value = -1;
        platform::set_errno(libc::EPERM);
}}