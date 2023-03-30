// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdatePackageVersionsStatus`](crate::client::fluent_builders::UpdatePackageVersionsStatus) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`domain(impl Into<String>)`](crate::client::fluent_builders::UpdatePackageVersionsStatus::domain) / [`set_domain(Option<String>)`](crate::client::fluent_builders::UpdatePackageVersionsStatus::set_domain): <p> The name of the domain that contains the repository that contains the package versions with a status to be updated. </p>
    ///   - [`domain_owner(impl Into<String>)`](crate::client::fluent_builders::UpdatePackageVersionsStatus::domain_owner) / [`set_domain_owner(Option<String>)`](crate::client::fluent_builders::UpdatePackageVersionsStatus::set_domain_owner): <p> The 12-digit account number of the Amazon Web Services account that owns the domain. It does not include dashes or spaces. </p>
    ///   - [`repository(impl Into<String>)`](crate::client::fluent_builders::UpdatePackageVersionsStatus::repository) / [`set_repository(Option<String>)`](crate::client::fluent_builders::UpdatePackageVersionsStatus::set_repository): <p> The repository that contains the package versions with the status you want to update. </p>
    ///   - [`format(PackageFormat)`](crate::client::fluent_builders::UpdatePackageVersionsStatus::format) / [`set_format(Option<PackageFormat>)`](crate::client::fluent_builders::UpdatePackageVersionsStatus::set_format): <p> A format that specifies the type of the package with the statuses to update. </p>
    ///   - [`namespace(impl Into<String>)`](crate::client::fluent_builders::UpdatePackageVersionsStatus::namespace) / [`set_namespace(Option<String>)`](crate::client::fluent_builders::UpdatePackageVersionsStatus::set_namespace): <p>The namespace of the package version to be updated. The package version component that specifies its namespace depends on its type. For example:</p>  <ul>   <li> <p> The namespace of a Maven package version is its <code>groupId</code>. </p> </li>   <li> <p> The namespace of an npm package version is its <code>scope</code>. </p> </li>   <li> <p> Python and NuGet package versions do not contain a corresponding component, package versions of those formats do not have a namespace. </p> </li>  </ul>
    ///   - [`package(impl Into<String>)`](crate::client::fluent_builders::UpdatePackageVersionsStatus::package) / [`set_package(Option<String>)`](crate::client::fluent_builders::UpdatePackageVersionsStatus::set_package): <p> The name of the package with the version statuses to update. </p>
    ///   - [`versions(Vec<String>)`](crate::client::fluent_builders::UpdatePackageVersionsStatus::versions) / [`set_versions(Option<Vec<String>>)`](crate::client::fluent_builders::UpdatePackageVersionsStatus::set_versions): <p> An array of strings that specify the versions of the package with the statuses to update. </p>
    ///   - [`version_revisions(HashMap<String, String>)`](crate::client::fluent_builders::UpdatePackageVersionsStatus::version_revisions) / [`set_version_revisions(Option<HashMap<String, String>>)`](crate::client::fluent_builders::UpdatePackageVersionsStatus::set_version_revisions): <p> A map of package versions and package version revisions. The map <code>key</code> is the package version (for example, <code>3.5.2</code>), and the map <code>value</code> is the package version revision. </p>
    ///   - [`expected_status(PackageVersionStatus)`](crate::client::fluent_builders::UpdatePackageVersionsStatus::expected_status) / [`set_expected_status(Option<PackageVersionStatus>)`](crate::client::fluent_builders::UpdatePackageVersionsStatus::set_expected_status): <p> The package version’s expected status before it is updated. If <code>expectedStatus</code> is provided, the package version's status is updated only if its status at the time <code>UpdatePackageVersionsStatus</code> is called matches <code>expectedStatus</code>. </p>
    ///   - [`target_status(PackageVersionStatus)`](crate::client::fluent_builders::UpdatePackageVersionsStatus::target_status) / [`set_target_status(Option<PackageVersionStatus>)`](crate::client::fluent_builders::UpdatePackageVersionsStatus::set_target_status): <p> The status you want to change the package version status to. </p>
                            /// - On success, responds with [`UpdatePackageVersionsStatusOutput`](crate::output::UpdatePackageVersionsStatusOutput) with field(s):
    ///   - [`successful_versions(Option<HashMap<String, SuccessfulPackageVersionInfo>>)`](crate::output::UpdatePackageVersionsStatusOutput::successful_versions): <p> A list of <code>PackageVersionError</code> objects, one for each package version with a status that failed to update. </p>
    ///   - [`failed_versions(Option<HashMap<String, PackageVersionError>>)`](crate::output::UpdatePackageVersionsStatusOutput::failed_versions): <p> A list of <code>SuccessfulPackageVersionInfo</code> objects, one for each package version with a status that successfully updated. </p>
                            /// - On failure, responds with [`SdkError<UpdatePackageVersionsStatusError>`](crate::error::UpdatePackageVersionsStatusError)
    pub fn update_package_versions_status(&self) -> crate::client::fluent_builders::UpdatePackageVersionsStatus {
                                crate::client::fluent_builders::UpdatePackageVersionsStatus::new(self.handle.clone())
                            }
}

