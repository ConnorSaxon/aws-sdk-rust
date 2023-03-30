// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DissociatePackage`](crate::client::fluent_builders::DissociatePackage) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`package_id(impl Into<String>)`](crate::client::fluent_builders::DissociatePackage::package_id) / [`set_package_id(Option<String>)`](crate::client::fluent_builders::DissociatePackage::set_package_id): <p>Internal ID of the package to dissociate from the domain. Use <code>ListPackagesForDomain</code> to find this value.</p>
    ///   - [`domain_name(impl Into<String>)`](crate::client::fluent_builders::DissociatePackage::domain_name) / [`set_domain_name(Option<String>)`](crate::client::fluent_builders::DissociatePackage::set_domain_name): <p>Name of the domain to dissociate the package from.</p>
                            /// - On success, responds with [`DissociatePackageOutput`](crate::output::DissociatePackageOutput) with field(s):
    ///   - [`domain_package_details(Option<DomainPackageDetails>)`](crate::output::DissociatePackageOutput::domain_package_details): <p> Information about a package that has been dissociated from the domain.</p>
                            /// - On failure, responds with [`SdkError<DissociatePackageError>`](crate::error::DissociatePackageError)
    pub fn dissociate_package(&self) -> crate::client::fluent_builders::DissociatePackage {
                                crate::client::fluent_builders::DissociatePackage::new(self.handle.clone())
                            }
}

