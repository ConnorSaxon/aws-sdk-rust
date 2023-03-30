// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetVpcLinks`](crate::client::fluent_builders::GetVpcLinks) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`max_results(impl Into<String>)`](crate::client::fluent_builders::GetVpcLinks::max_results) / [`set_max_results(Option<String>)`](crate::client::fluent_builders::GetVpcLinks::set_max_results): <p>The maximum number of elements to be returned for this resource.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetVpcLinks::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetVpcLinks::set_next_token): <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
                            /// - On success, responds with [`GetVpcLinksOutput`](crate::output::GetVpcLinksOutput) with field(s):
    ///   - [`items(Option<Vec<VpcLink>>)`](crate::output::GetVpcLinksOutput::items): <p>A collection of VPC links.</p>
    ///   - [`next_token(Option<String>)`](crate::output::GetVpcLinksOutput::next_token): <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
                            /// - On failure, responds with [`SdkError<GetVpcLinksError>`](crate::error::GetVpcLinksError)
    pub fn get_vpc_links(&self) -> crate::client::fluent_builders::GetVpcLinks {
                                crate::client::fluent_builders::GetVpcLinks::new(self.handle.clone())
                            }
}

