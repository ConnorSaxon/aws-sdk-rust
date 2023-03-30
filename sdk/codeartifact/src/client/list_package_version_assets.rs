// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListPackageVersionAssets`](crate::client::fluent_builders::ListPackageVersionAssets) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListPackageVersionAssets::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`domain(impl Into<String>)`](crate::client::fluent_builders::ListPackageVersionAssets::domain) / [`set_domain(Option<String>)`](crate::client::fluent_builders::ListPackageVersionAssets::set_domain): <p> The name of the domain that contains the repository associated with the package version assets. </p>
    ///   - [`domain_owner(impl Into<String>)`](crate::client::fluent_builders::ListPackageVersionAssets::domain_owner) / [`set_domain_owner(Option<String>)`](crate::client::fluent_builders::ListPackageVersionAssets::set_domain_owner): <p> The 12-digit account number of the Amazon Web Services account that owns the domain. It does not include dashes or spaces. </p>
    ///   - [`repository(impl Into<String>)`](crate::client::fluent_builders::ListPackageVersionAssets::repository) / [`set_repository(Option<String>)`](crate::client::fluent_builders::ListPackageVersionAssets::set_repository): <p> The name of the repository that contains the package that contains the requested package version assets. </p>
    ///   - [`format(PackageFormat)`](crate::client::fluent_builders::ListPackageVersionAssets::format) / [`set_format(Option<PackageFormat>)`](crate::client::fluent_builders::ListPackageVersionAssets::set_format): <p> The format of the package that contains the requested package version assets. </p>
    ///   - [`namespace(impl Into<String>)`](crate::client::fluent_builders::ListPackageVersionAssets::namespace) / [`set_namespace(Option<String>)`](crate::client::fluent_builders::ListPackageVersionAssets::set_namespace): <p>The namespace of the package version that contains the requested package version assets. The package version component that specifies its namespace depends on its type. For example:</p>  <ul>   <li> <p> The namespace of a Maven package version is its <code>groupId</code>. </p> </li>   <li> <p> The namespace of an npm package version is its <code>scope</code>. </p> </li>   <li> <p> Python and NuGet package versions do not contain a corresponding component, package versions of those formats do not have a namespace. </p> </li>  </ul>
    ///   - [`package(impl Into<String>)`](crate::client::fluent_builders::ListPackageVersionAssets::package) / [`set_package(Option<String>)`](crate::client::fluent_builders::ListPackageVersionAssets::set_package): <p> The name of the package that contains the requested package version assets. </p>
    ///   - [`package_version(impl Into<String>)`](crate::client::fluent_builders::ListPackageVersionAssets::package_version) / [`set_package_version(Option<String>)`](crate::client::fluent_builders::ListPackageVersionAssets::set_package_version): <p> A string that contains the package version (for example, <code>3.5.2</code>). </p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListPackageVersionAssets::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListPackageVersionAssets::set_max_results): <p> The maximum number of results to return per page. </p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListPackageVersionAssets::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListPackageVersionAssets::set_next_token): <p> The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results. </p>
                            /// - On success, responds with [`ListPackageVersionAssetsOutput`](crate::output::ListPackageVersionAssetsOutput) with field(s):
    ///   - [`format(Option<PackageFormat>)`](crate::output::ListPackageVersionAssetsOutput::format): <p> The format of the package that contains the requested package version assets. </p>
    ///   - [`namespace(Option<String>)`](crate::output::ListPackageVersionAssetsOutput::namespace): <p>The namespace of the package version that contains the requested package version assets. The package version component that specifies its namespace depends on its type. For example:</p>  <ul>   <li> <p> The namespace of a Maven package version is its <code>groupId</code>. </p> </li>   <li> <p> The namespace of an npm package version is its <code>scope</code>. </p> </li>   <li> <p> Python and NuGet package versions do not contain a corresponding component, package versions of those formats do not have a namespace. </p> </li>  </ul>
    ///   - [`package(Option<String>)`](crate::output::ListPackageVersionAssetsOutput::package): <p> The name of the package that contains the requested package version assets. </p>
    ///   - [`version(Option<String>)`](crate::output::ListPackageVersionAssetsOutput::version): <p> The version of the package associated with the requested assets. </p>
    ///   - [`version_revision(Option<String>)`](crate::output::ListPackageVersionAssetsOutput::version_revision): <p> The current revision associated with the package version. </p>
    ///   - [`next_token(Option<String>)`](crate::output::ListPackageVersionAssetsOutput::next_token): <p> If there are additional results, this is the token for the next set of results. </p>
    ///   - [`assets(Option<Vec<AssetSummary>>)`](crate::output::ListPackageVersionAssetsOutput::assets): <p> The returned list of <a href="https://docs.aws.amazon.com/codeartifact/latest/APIReference/API_AssetSummary.html">AssetSummary</a> objects. </p>
                            /// - On failure, responds with [`SdkError<ListPackageVersionAssetsError>`](crate::error::ListPackageVersionAssetsError)
    pub fn list_package_version_assets(&self) -> crate::client::fluent_builders::ListPackageVersionAssets {
                                crate::client::fluent_builders::ListPackageVersionAssets::new(self.handle.clone())
                            }
}

