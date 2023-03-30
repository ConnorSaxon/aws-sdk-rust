// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeImagePermissions`](crate::client::fluent_builders::DescribeImagePermissions) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeImagePermissions::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::DescribeImagePermissions::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::DescribeImagePermissions::set_name): <p>The name of the private image for which to describe permissions. The image must be one that you own. </p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeImagePermissions::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeImagePermissions::set_max_results): <p>The maximum size of each page of results.</p>
    ///   - [`shared_aws_account_ids(Vec<String>)`](crate::client::fluent_builders::DescribeImagePermissions::shared_aws_account_ids) / [`set_shared_aws_account_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeImagePermissions::set_shared_aws_account_ids): <p>The 12-digit identifier of one or more AWS accounts with which the image is shared.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeImagePermissions::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeImagePermissions::set_next_token): <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
                            /// - On success, responds with [`DescribeImagePermissionsOutput`](crate::output::DescribeImagePermissionsOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::output::DescribeImagePermissionsOutput::name): <p>The name of the private image.</p>
    ///   - [`shared_image_permissions_list(Option<Vec<SharedImagePermissions>>)`](crate::output::DescribeImagePermissionsOutput::shared_image_permissions_list): <p>The permissions for a private image that you own. </p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeImagePermissionsOutput::next_token): <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
                            /// - On failure, responds with [`SdkError<DescribeImagePermissionsError>`](crate::error::DescribeImagePermissionsError)
    pub fn describe_image_permissions(&self) -> crate::client::fluent_builders::DescribeImagePermissions {
                                crate::client::fluent_builders::DescribeImagePermissions::new(self.handle.clone())
                            }
}

