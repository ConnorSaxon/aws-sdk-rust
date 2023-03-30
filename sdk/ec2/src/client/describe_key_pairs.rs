// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeKeyPairs`](crate::client::fluent_builders::DescribeKeyPairs) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::DescribeKeyPairs::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::DescribeKeyPairs::set_filters): <p>The filters.</p>  <ul>   <li> <p> <code>key-pair-id</code> - The ID of the key pair.</p> </li>   <li> <p> <code>fingerprint</code> - The fingerprint of the key pair.</p> </li>   <li> <p> <code>key-name</code> - The name of the key pair.</p> </li>   <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>   <li> <p> <code>tag</code>:<key>      - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key      <code>Owner</code> and the value      <code>TeamA</code>, specify      <code>tag:Owner</code> for the filter name and      <code>TeamA</code> for the filter value.    </key></p> </li>  </ul>
    ///   - [`key_names(Vec<String>)`](crate::client::fluent_builders::DescribeKeyPairs::key_names) / [`set_key_names(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeKeyPairs::set_key_names): <p>The key pair names.</p>  <p>Default: Describes all of your key pairs.</p>
    ///   - [`key_pair_ids(Vec<String>)`](crate::client::fluent_builders::DescribeKeyPairs::key_pair_ids) / [`set_key_pair_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeKeyPairs::set_key_pair_ids): <p>The IDs of the key pairs.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DescribeKeyPairs::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DescribeKeyPairs::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`include_public_key(bool)`](crate::client::fluent_builders::DescribeKeyPairs::include_public_key) / [`set_include_public_key(Option<bool>)`](crate::client::fluent_builders::DescribeKeyPairs::set_include_public_key): <p>If <code>true</code>, the public key material is included in the response.</p>  <p>Default: <code>false</code> </p>
                            /// - On success, responds with [`DescribeKeyPairsOutput`](crate::output::DescribeKeyPairsOutput) with field(s):
    ///   - [`key_pairs(Option<Vec<KeyPairInfo>>)`](crate::output::DescribeKeyPairsOutput::key_pairs): <p>Information about the key pairs.</p>
                            /// - On failure, responds with [`SdkError<DescribeKeyPairsError>`](crate::error::DescribeKeyPairsError)
    pub fn describe_key_pairs(&self) -> crate::client::fluent_builders::DescribeKeyPairs {
                                crate::client::fluent_builders::DescribeKeyPairs::new(self.handle.clone())
                            }
}

