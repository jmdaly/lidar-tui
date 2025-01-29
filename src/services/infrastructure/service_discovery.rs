#![expect(dead_code)]

use std::error::Error;

use mdns_sd::Receiver;
use mdns_sd::ServiceDaemon;
use mdns_sd::ServiceEvent;
use mdns_sd::ServiceInfo;

/// An enumeration of service discovery events.
///
/// `AppServiceEvent` is an interface wrapper over the [`ServiceEvent`](mdns_sd::ServiceEvent) enumeration
/// from the `mdns-sd` crate, providing a simplified and application-specific view of service discovery events.
enum AppServiceEvent {
    SearchStarted(String),
    ServiceFound(String, String),
    ServiceResolved(ServiceInfo),
    ServiceRemoved(String, String),
    SearchStopped(String),
}

/// Converts a `ServiceEvent` into an `AppServiceEvent`.
impl From<ServiceEvent> for AppServiceEvent {
    fn from(event: ServiceEvent) -> Self {
        match event {
            ServiceEvent::SearchStarted(service_type) => {
                AppServiceEvent::SearchStarted(service_type)
            }
            ServiceEvent::ServiceFound(service_type, name) => {
                AppServiceEvent::ServiceFound(service_type, name)
            }
            ServiceEvent::ServiceResolved(info) => AppServiceEvent::ServiceResolved(info),
            ServiceEvent::ServiceRemoved(service_type, name) => {
                AppServiceEvent::ServiceRemoved(service_type, name)
            }
            ServiceEvent::SearchStopped(service_type) => {
                AppServiceEvent::SearchStopped(service_type)
            }
        }
    }
}

/// Trait for receiving service discovery events.
///
/// `ServiceEventReceiver` defines an interface for types that can receive service discovery events.
trait ServiceEventReceiver {
    fn receive(&self) -> Result<AppServiceEvent, Box<dyn Error>>;
}

/// An interface wrapper for receiving service discovery events from the `mdns-sd` crate.
///
/// `MdnsServiceEventReceiver` wraps a [`Receiver`](mdns_sd::Receiver) from the `mdns-sd` crate
/// and implements the `ServiceEventReceiver` trait, allowing for the reception of service discovery events.
struct MdnsServiceEventReceiver(Receiver<ServiceEvent>);

impl ServiceEventReceiver for MdnsServiceEventReceiver {
    fn receive(&self) -> Result<AppServiceEvent, Box<dyn Error>> {
        let event = self.0.recv()?;
        Ok(event.into())
    }
}

/// An embedded stub implementation of `ServiceEventReceiver` for testing purposes.
///
/// `StubServiceEventReceiver` is a stub implementation of the `ServiceEventReceiver` trait.
/// It is designed for use in testing scenarios where actual service discovery is not required
/// and provides a predictable stream of `AppServiceEvent`s.
struct StubServiceEventReceiver {}

impl ServiceEventReceiver for StubServiceEventReceiver {
    fn receive(&self) -> Result<AppServiceEvent, Box<dyn Error>> {
        Ok(AppServiceEvent::SearchStarted("test".to_string()))
    }
}

/// Trait for mDNS service discovery operations.
/// Implementations of this trait provide methods to browse for available services
/// on the network using multicast DNS.
trait MdnsDiscoveryTrait<T: ServiceEventReceiver> {
    /// Returns a list of discovered service names.
    ///
    /// # Returns
    /// A vector of strings containing the names of discovered services.
    fn browse(&self, service_type: &str) -> Result<T, Box<dyn Error>>;
}

/// An interface wrapper for performing mDNS service discovery using the `mdns-sd` crate.
///
/// `MdnsDiscovery` wraps the [`ServiceDaemon`](mdns_sd::ServiceDaemon) from the `mdns-sd` crate
/// and implements the `MdnsDiscoveryTrait`. It provides methods to browse for network services
/// using mDNS.
struct MdnsDiscovery(ServiceDaemon);

impl MdnsDiscovery {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(MdnsDiscovery(ServiceDaemon::new()?))
    }
}

impl MdnsDiscoveryTrait<MdnsServiceEventReceiver> for MdnsDiscovery {
    fn browse(&self, service_type: &str) -> Result<MdnsServiceEventReceiver, Box<dyn Error>> {
        // Ok(self.0.browse(service_type)?)
        Err("Not implemented".into())
    }
}

/// An embedded stub implementation of `MdnsDiscoveryTrait` for testing purposes.
///
/// `StubMdnsDiscovery` is a stub implementation of the `MdnsDiscoveryTrait`.
/// It is designed for use in testing scenarios where actual service discovery is not required.
/// It does not perform any real mDNS operations.
struct StubMdnsDiscovery {}

impl MdnsDiscoveryTrait<StubServiceEventReceiver> for StubMdnsDiscovery {
    fn browse(&self, _service_type: &str) -> Result<StubServiceEventReceiver, Box<dyn Error>> {
        Err("Not implemented".into())
    }
}

/// A service discovery component that uses mDNS to discover network services.
///
/// `ServiceDiscovery` is a generic component that utilizes a concrete implementation of `MdnsDiscoveryTrait`
/// to perform service discovery. It abstracts away the underlying mDNS implementation, allowing for
/// different discovery mechanisms to be used.
///
/// # Type Parameters
///
/// * `M` - A type that implements the `MdnsDiscoveryTrait` for service discovery operations.
/// * `T` - A type that implements `ServiceEventReceiver` to receive service discovery events.
struct ServiceDiscovery<M: MdnsDiscoveryTrait<T>, T: ServiceEventReceiver> {
    mdns_discovery: M,
    _phantom: std::marker::PhantomData<T>,
}

impl<M: MdnsDiscoveryTrait<T>, T: ServiceEventReceiver> ServiceDiscovery<M, T> {
    pub fn browse(&self, service_type: &str) -> Result<MdnsServiceEventReceiver, Box<dyn Error>> {
        // self.mdns_discovery.browse(service_type)
        Err("Not implemented".into())
    }
}

impl ServiceDiscovery<MdnsDiscovery, MdnsServiceEventReceiver> {
    /// Creates a new `ServiceDiscovery` instance using the production mDNS implementation.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - A new `ServiceDiscovery` instance.
    /// * `Err(Box<dyn Error>)` - If an error occurred during initialization.
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(ServiceDiscovery {
            mdns_discovery: MdnsDiscovery(ServiceDaemon::new()?),
            _phantom: std::marker::PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl ServiceDiscovery<StubMdnsDiscovery, StubServiceEventReceiver> {
        /// Creates a new `ServiceDiscovery` instance with stub implementations for testing or null
        /// object pattern.
        ///
        /// This constructor is intended for use in testing scenarios where actual service
        /// discovery is not needed. It creates a `ServiceDiscovery` instance that uses
        /// `StubMdnsDiscovery` and `StubServiceEventReceiver`, providing a controlled environment
        /// for tests.
        pub fn new_null() -> Self {
            ServiceDiscovery {
                mdns_discovery: StubMdnsDiscovery {},
                _phantom: std::marker::PhantomData,
            }
        }
    }

    #[test]
    fn no_available_services() {
        let service_discovery = ServiceDiscovery::new_null();

        let services = service_discovery.browse("no_services._tcp.local.");
        assert!(services.is_err());
    }
}
