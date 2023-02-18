use std::{net::{IpAddr, SocketAddr}, num::NonZeroU16, borrow::Cow};

fn main() {
    println!("Refactor me!");

    let mut err = Error::new("NO_USER".into());
    err.status(NonZeroU16::new(404).unwrap()).message("User not found");
}

#[derive(Debug)]
pub struct Error<'a, 'b> {
    code: Cow<'a, str>,
    status: u16,
    message: Cow<'b, str>,
}

impl <'a, 'b>Default for Error<'a, 'b> {
    #[inline]
    fn default() -> Self {
        Self {
            code: "UNKNOWN".into(),
            status: 500,
            message: "Unknown error has happened.".into(),
        }
    }
}

impl <'a, 'b>Error<'a, 'b> {
    pub fn new(code: Cow<'a, str>) -> Self {
        let mut err = Self::default();
        err.code = code.into();
        err
    }

    pub fn status<S: Into<u16>>(&mut self, s: S) -> &mut Self {
        self.status = s.into();
        self
    }

    pub fn message<S: Into<Cow<'b, str>>>(&mut self, m: S) -> &mut Self {
        self.message = m.into();
        self
    }
}

#[derive(Debug, Default)]
pub struct Server(Option<SocketAddr>);

impl Server {
    pub fn bind<U: Into<IpAddr>, V: Into<u16>>(&mut self, ip: U, port: V) {
        self.0 = Some(SocketAddr::new(ip.into(), port.into()))
    }
}

#[cfg(test)]
mod server_spec {
    use super::*;

    mod bind {
        use std::net::{Ipv4Addr, Ipv6Addr};

        use super::*;

        #[test]
        fn sets_provided_address_to_server() {
            let mut server = Server::default();

            server.bind([127, 0, 0, 1], 8080 as u16);
            assert_eq!(format!("{}", server.0.unwrap()), "127.0.0.1:8080");

            server.bind("::1".parse::<Ipv6Addr>().unwrap(), 9911 as u16);
            assert_eq!(format!("{}", server.0.unwrap()), "[::1]:9911");
        }
    }
}
