// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListPackages`](crate::client::fluent_builders::ListPackages) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListPackages::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`domain(impl Into<String>)`](crate::client::fluent_builders::ListPackages::domain) / [`set_domain(Option<String>)`](crate::client::fluent_builders::ListPackages::set_domain): <p> The name of the domain that contains the repository that contains the requested packages. </p>
    ///   - [`domain_owner(impl Into<String>)`](crate::client::fluent_builders::ListPackages::domain_owner) / [`set_domain_owner(Option<String>)`](crate::client::fluent_builders::ListPackages::set_domain_owner): <p> The 12-digit account number of the Amazon Web Services account that owns the domain. It does not include dashes or spaces. </p>
    ///   - [`repository(impl Into<String>)`](crate::client::fluent_builders::ListPackages::repository) / [`set_repository(Option<String>)`](crate::client::fluent_builders::ListPackages::set_repository): <p> The name of the repository that contains the requested packages. </p>
    ///   - [`format(PackageFormat)`](crate::client::fluent_builders::ListPackages::format) / [`set_format(Option<PackageFormat>)`](crate::client::fluent_builders::ListPackages::set_format): <p>The format used to filter requested packages. Only packages from the provided format will be returned.</p>
    ///   - [`namespace(impl Into<String>)`](crate::client::fluent_builders::ListPackages::namespace) / [`set_namespace(Option<String>)`](crate::client::fluent_builders::ListPackages::set_namespace): <p>The namespace prefix used to filter requested packages. Only packages with a namespace that starts with the provided string value are returned. Note that although this option is called <code>--namespace</code> and not <code>--namespace-prefix</code>, it has prefix-matching behavior.</p>  <p>Each package format uses namespace as follows:</p>  <ul>   <li> <p> The namespace of a Maven package is its <code>groupId</code>. </p> </li>   <li> <p> The namespace of an npm package is its <code>scope</code>. </p> </li>   <li> <p> Python and NuGet packages do not contain a corresponding component, packages of those formats do not have a namespace. </p> </li>  </ul>
    ///   - [`package_prefix(impl Into<String>)`](crate::client::fluent_builders::ListPackages::package_prefix) / [`set_package_prefix(Option<String>)`](crate::client::fluent_builders::ListPackages::set_package_prefix): <p> A prefix used to filter requested packages. Only packages with names that start with <code>packagePrefix</code> are returned. </p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListPackages::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListPackages::set_max_results): <p> The maximum number of results to return per page. </p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListPackages::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListPackages::set_next_token): <p> The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results. </p>
    ///   - [`publish(AllowPublish)`](crate::client::fluent_builders::ListPackages::publish) / [`set_publish(Option<AllowPublish>)`](crate::client::fluent_builders::ListPackages::set_publish): <p>The value of the <code>Publish</code> package origin control restriction used to filter requested packages. Only packages with the provided restriction are returned. For more information, see <a href="https://docs.aws.amazon.com/codeartifact/latest/APIReference/API_PackageOriginRestrictions.html">PackageOriginRestrictions</a>.</p>
    ///   - [`upstream(AllowUpstream)`](crate::client::fluent_builders::ListPackages::upstream) / [`set_upstream(Option<AllowUpstream>)`](crate::client::fluent_builders::ListPackages::set_upstream): <p>The value of the <code>Upstream</code> package origin control restriction used to filter requested packages. Only packages with the provided restriction are returned. For more information, see <a href="https://docs.aws.amazon.com/codeartifact/latest/APIReference/API_PackageOriginRestrictions.html">PackageOriginRestrictions</a>.</p>
                            /// - On success, responds with [`ListPackagesOutput`](crate::output::ListPackagesOutput) with field(s):
    ///   - [`packages(Option<Vec<PackageSummary>>)`](crate::output::ListPackagesOutput::packages): <p> The list of returned <a href="https://docs.aws.amazon.com/codeartifact/latest/APIReference/API_PackageSummary.html">PackageSummary</a> objects. </p>
    ///   - [`next_token(Option<String>)`](crate::output::ListPackagesOutput::next_token): <p> If there are additional results, this is the token for the next set of results. </p>
                            /// - On failure, responds with [`SdkError<ListPackagesError>`](crate::error::ListPackagesError)
    pub fn list_packages(&self) -> crate::client::fluent_builders::ListPackages {
                                crate::client::fluent_builders::ListPackages::new(self.handle.clone())
                            }
}

