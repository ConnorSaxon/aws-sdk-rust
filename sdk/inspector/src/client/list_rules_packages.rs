// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListRulesPackages`](crate::client::fluent_builders::ListRulesPackages) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListRulesPackages::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListRulesPackages::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListRulesPackages::set_next_token): <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the <b>ListRulesPackages</b> action. Subsequent calls to the action fill <b>nextToken</b> in the request with the value of <b>NextToken</b> from the previous response to continue listing data.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListRulesPackages::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListRulesPackages::set_max_results): <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 10. The maximum value is 500.</p>
                            /// - On success, responds with [`ListRulesPackagesOutput`](crate::output::ListRulesPackagesOutput) with field(s):
    ///   - [`rules_package_arns(Option<Vec<String>>)`](crate::output::ListRulesPackagesOutput::rules_package_arns): <p>The list of ARNs that specifies the rules packages returned by the action.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListRulesPackagesOutput::next_token): <p> When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the <b>nextToken</b> parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null.</p>
                            /// - On failure, responds with [`SdkError<ListRulesPackagesError>`](crate::error::ListRulesPackagesError)
    pub fn list_rules_packages(&self) -> crate::client::fluent_builders::ListRulesPackages {
                                crate::client::fluent_builders::ListRulesPackages::new(self.handle.clone())
                            }
}

