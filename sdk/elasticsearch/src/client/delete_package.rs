// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeletePackage`](crate::client::fluent_builders::DeletePackage) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`package_id(impl Into<String>)`](crate::client::fluent_builders::DeletePackage::package_id) / [`set_package_id(Option<String>)`](crate::client::fluent_builders::DeletePackage::set_package_id): <p>Internal ID of the package that you want to delete. Use <code>DescribePackages</code> to find this value.</p>
                            /// - On success, responds with [`DeletePackageOutput`](crate::output::DeletePackageOutput) with field(s):
    ///   - [`package_details(Option<PackageDetails>)`](crate::output::DeletePackageOutput::package_details): <p><code>PackageDetails</code></p>
                            /// - On failure, responds with [`SdkError<DeletePackageError>`](crate::error::DeletePackageError)
    pub fn delete_package(&self) -> crate::client::fluent_builders::DeletePackage {
                                crate::client::fluent_builders::DeletePackage::new(self.handle.clone())
                            }
}

