#![expect(dead_code)]

use std::error::Error;

use mdns_sd::ServiceDaemon;

/// Trait for mDNS service discovery operations.
/// Implementations of this trait provide methods to browse for available services
/// on the network using multicast DNS.
trait MdnsDiscoveryTrait {
    /// Returns a list of discovered service names.
    ///
    /// # Returns
    /// A vector of strings containing the names of discovered services.
    fn browse(&self) -> Vec<String>;
}

/// An implementation of `MdnsDiscoveryTrait` that uses the mdns-sd crate's `ServiceDaemon`
/// for performing actual mDNS service discovery operations.
struct MdnsDiscovery(ServiceDaemon);

impl MdnsDiscoveryTrait for MdnsDiscovery {
    fn browse(&self) -> Vec<String> {
        Vec::new()
    }
}

/// A service discovery component that uses mDNS to discover network services.
///
/// # Type Parameters
/// * `M` - A type that implements the `MdnsDiscoveryTrait` for service discovery operations
struct ServiceDiscovery<M: MdnsDiscoveryTrait> {
    mdns_discovery: M,
}

impl<M: MdnsDiscoveryTrait> ServiceDiscovery<M> {
    pub fn browse(&self) -> Vec<String> {
        self.mdns_discovery.browse()
    }
}

impl ServiceDiscovery<MdnsDiscovery> {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(ServiceDiscovery {
            mdns_discovery: MdnsDiscovery(ServiceDaemon::new()?),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A stub implementation of `MdnsDiscoveryTrait` used for testing purposes.
    struct StubMdnsDiscovery {}

    impl MdnsDiscoveryTrait for StubMdnsDiscovery {
        fn browse(&self) -> Vec<String> {
            Vec::new()
        }
    }

    impl ServiceDiscovery<StubMdnsDiscovery> {
        pub fn new_null() -> Self {
            ServiceDiscovery {
                mdns_discovery: StubMdnsDiscovery {},
            }
        }
    }

    #[test]
    fn no_available_services() {
        let service_discovery = ServiceDiscovery::new_null();

        let services = service_discovery.browse();
        assert!(services.is_empty());
    }
}
