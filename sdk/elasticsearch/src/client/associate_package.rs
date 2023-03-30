// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AssociatePackage`](crate::client::fluent_builders::AssociatePackage) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`package_id(impl Into<String>)`](crate::client::fluent_builders::AssociatePackage::package_id) / [`set_package_id(Option<String>)`](crate::client::fluent_builders::AssociatePackage::set_package_id): <p>Internal ID of the package that you want to associate with a domain. Use <code>DescribePackages</code> to find this value.</p>
    ///   - [`domain_name(impl Into<String>)`](crate::client::fluent_builders::AssociatePackage::domain_name) / [`set_domain_name(Option<String>)`](crate::client::fluent_builders::AssociatePackage::set_domain_name): <p>Name of the domain that you want to associate the package with.</p>
                            /// - On success, responds with [`AssociatePackageOutput`](crate::output::AssociatePackageOutput) with field(s):
    ///   - [`domain_package_details(Option<DomainPackageDetails>)`](crate::output::AssociatePackageOutput::domain_package_details): <p><code>DomainPackageDetails</code></p>
                            /// - On failure, responds with [`SdkError<AssociatePackageError>`](crate::error::AssociatePackageError)
    pub fn associate_package(&self) -> crate::client::fluent_builders::AssociatePackage {
                                crate::client::fluent_builders::AssociatePackage::new(self.handle.clone())
                            }
}

