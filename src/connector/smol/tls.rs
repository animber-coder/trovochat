use super::*;

/// A `smol` connector that uses `async-tls` (a `rustls` wrapper). This uses TLS.
///
/// To use this type, ensure you set up the 'TLS Domain' in the
/// configuration. The crate provides the 'TLS domain' for Trovo in the root of this crate.
#[derive(Debug, Clone, PartialEq)]
pub struct ConnectorTls {
    addrs: Vec<std::net::SocketAddr>,
    tls_domain: String,
}

impl ConnectorTls {
    connector_ctor!(tls:
        /// [`smol`](https://docs.rs/smol/latest/smol/)
    );
}

impl crate::connector::Connector for ConnectorTls {
    type Output = async_dup::Mutex<async_tls::client::TlsStream<TcpStream>>;

    fn connect(&mut self) -> BoxedFuture<std::io::Result<Self::Output>> {
        let this = self.clone();
        let fut = async move {
            let stream = try_connect(&*this.addrs, TcpStream::connect).await?;
            async_tls::TlsConnector::new()
                .connect(this.tls_domain, stream)
                .await
                .map(async_dup::Mutex::new)
        };
        Box::pin(fut)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_connector_trait_is_fulfilled() {
        use crate::connector::testing::*;
        use crate::connector::Connector as C;

        assert_connector::<ConnectorTls>();
        assert_type_is_read_write::<<ConnectorTls as C>::Output>();
        assert_obj_is_sane(ConnectorTls::trovo().unwrap());
    }
}
