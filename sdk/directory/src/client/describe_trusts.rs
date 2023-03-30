// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeTrusts`](crate::client::fluent_builders::DescribeTrusts) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeTrusts::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`directory_id(impl Into<String>)`](crate::client::fluent_builders::DescribeTrusts::directory_id) / [`set_directory_id(Option<String>)`](crate::client::fluent_builders::DescribeTrusts::set_directory_id): <p>The Directory ID of the Amazon Web Services directory that is a part of the requested trust relationship.</p>
    ///   - [`trust_ids(Vec<String>)`](crate::client::fluent_builders::DescribeTrusts::trust_ids) / [`set_trust_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeTrusts::set_trust_ids): <p>A list of identifiers of the trust relationships for which to obtain the information. If this member is null, all trust relationships that belong to the current account are returned.</p>  <p>An empty list results in an <code>InvalidParameterException</code> being thrown.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeTrusts::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeTrusts::set_next_token): <p>The <i>DescribeTrustsResult.NextToken</i> value from a previous call to <code>DescribeTrusts</code>. Pass null if this is the first call.</p>
    ///   - [`limit(i32)`](crate::client::fluent_builders::DescribeTrusts::limit) / [`set_limit(Option<i32>)`](crate::client::fluent_builders::DescribeTrusts::set_limit): <p>The maximum number of objects to return.</p>
                            /// - On success, responds with [`DescribeTrustsOutput`](crate::output::DescribeTrustsOutput) with field(s):
    ///   - [`trusts(Option<Vec<Trust>>)`](crate::output::DescribeTrustsOutput::trusts): <p>The list of Trust objects that were retrieved.</p>  <p>It is possible that this list contains less than the number of items specified in the <i>Limit</i> member of the request. This occurs if there are less than the requested number of items left to retrieve, or if the limitations of the operation have been exceeded.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeTrustsOutput::next_token): <p>If not null, more results are available. Pass this value for the <i>NextToken</i> parameter in a subsequent call to <code>DescribeTrusts</code> to retrieve the next set of items.</p>
                            /// - On failure, responds with [`SdkError<DescribeTrustsError>`](crate::error::DescribeTrustsError)
    pub fn describe_trusts(&self) -> crate::client::fluent_builders::DescribeTrusts {
                                crate::client::fluent_builders::DescribeTrusts::new(self.handle.clone())
                            }
}

