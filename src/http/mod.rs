use std::any::Any;

use axum::http::{header, HeaderName, Request};

use crate::configuration::ServerSettings;

// From https://docs.rs/actix-web/3.3.2/src/actix_web/info.rs.html#19-188
const X_FORWARDED_FOR: &[u8] = b"x-forwarded-for";
const X_FORWARDED_HOST: &[u8] = b"x-forwarded-host";
const X_FORWARDED_PROTO: &[u8] = b"x-forwarded-proto";

pub struct ConnectionInfo {
    scheme: String,
    host: String,
    realip_remote_addr: Option<String>,
    remote_addr: Option<String>,
}

impl ConnectionInfo {
    fn new<T>(req: Request<T>, cfg: &ServerSettings) {
        let mut host = None;
        let mut scheme = None;
        let mut realip_remote_addr = None;

        // parse forwarded header: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Forwarded
        // Syntax:
        // Forwarded: by=<identifier>;for=<identifier>;host=<host>;proto=<http|https>
        for hdr in req.headers().get_all(&header::FORWARDED) {
            // convert forwarded header to string if exists
            if let Ok(val) = hdr.to_str() {
                for pair in val.split(';') {
                    for el in pair.split(',') {
                        let mut items = el.trim().splitn(2, '=');
                        if let Some(name) = items.next() {
                            if let Some(val) = items.next() {
                                match &name.to_lowercase() as &str {
                                    "for" => {
                                        if realip_remote_addr.is_none() {
                                            realip_remote_addr = Some(val.trim());
                                        }
                                    }
                                    "proto" => {
                                        if scheme.is_none() {
                                            scheme = Some(val.trim());
                                        }
                                    }
                                    "host" => {
                                        if host.is_none() {
                                            host = Some(val.trim());
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                }
            }
        }

        // If scheme wasn't parsed from forwarded header
        // parse from x_forwarded_proto: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Forwarded-For
        // Syntax: X-Forwarded-Proto: <protocol>
        // If no forwarded header, parse from uri
        // If no uri then set it to https if that is the config
        if scheme.is_none() {
            if let Some(h) = req
                .headers()
                .get(&HeaderName::from_lowercase(X_FORWARDED_PROTO).unwrap())
            {
                if let Ok(h) = h.to_str() {
                    scheme = h.split(',').next().map(|v| v.trim());
                }
            }
            if scheme.is_none() {
                scheme = req.uri().scheme().map(|a| a.as_str());
                if scheme.is_none() && cfg.secure() {
                    scheme = Some("https")
                }
            }
        }

        // If host not in forwarded, parse x_forwarded_host: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Forwarded-Host
        // Syntax: X-Forwarded-Host: <host>
        // If not X-Forwarded-Host parse host header: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Host
        // Syntax: Host: <host>:<port>
        // If no host header, get from Uri
        // If can't get from URI set to serversettings host
        if host.is_none() {
            if let Some(h) = req
                .headers()
                .get(&HeaderName::from_lowercase(X_FORWARDED_HOST).unwrap())
            {
                if let Ok(h) = h.to_str() {
                    host = h.split(',').next().map(|v| v.trim());
                }
            }
            if host.is_none() {
                if let Some(h) = req.headers().get(&header::HOST) {
                    host = h.to_str().ok();
                }
                if host.is_none() {
                    host = req.uri().authority().map(|a| a.as_str());
                    if host.is_none() {
                        host = Some(cfg.host());
                    }
                }
            }
        }

        // get remote_addr from socketaddr
        // let remote_addr = req.peer_addr.ma
        //
    }

    /// Scheme of the request.
    ///
    /// Scheme is resolved through following headers, in order:
    ///
    /// 1. Forwarded
    /// 2. X-Forwarded-Proto
    /// 3. Uri
    pub fn scheme(&self) -> &str {
        &self.scheme
    }

    /// Hostname of the request
    ///
    /// Hostname is resolved through the following headers, in order:
    ///
    /// 1. Forwarded
    /// 2. X-Forwarded-Host
    /// 3. Host
    /// 4. Uri
    /// 5. Server hostname
    pub fn host(&self) -> &str {
        &self.host
    }

    /// remote_addr address of the request
    ///
    /// Get the remote_addr address from the socket address
    pub fn remote_addr(&self) -> Option<&str> {
        if let Some(ref remote_addr) = self.remote_addr {
            Some(remote_addr)
        } else {
            None
        }
    }

    /// Real ip remote addr of client initiated HTTP request
    ///
    /// Address is resolved through following headers, in order:
    ///
    /// 1. Forwarded
    /// 2. X-Forwarded-For
    /// 3. remote_addr name of opened socket
    ///
    /// # Security
    /// Do not use this function for security purposes, unless you can ensure the
    /// Forwarded and X-Forwarded-For headers cannot be spoofed by the client.
    pub fn realip_remote_addr(&self) -> Option<&str> {
        if let Some(ref r) = self.realip_remote_addr {
            Some(r)
        } else if let Some(ref remote_addr) = self.remote_addr {
            Some(remote_addr)
        } else {
            None
        }
    }
}
