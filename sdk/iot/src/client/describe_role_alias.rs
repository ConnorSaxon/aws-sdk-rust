// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeRoleAlias`](crate::client::fluent_builders::DescribeRoleAlias) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`role_alias(impl Into<String>)`](crate::client::fluent_builders::DescribeRoleAlias::role_alias) / [`set_role_alias(Option<String>)`](crate::client::fluent_builders::DescribeRoleAlias::set_role_alias): <p>The role alias to describe.</p>
                            /// - On success, responds with [`DescribeRoleAliasOutput`](crate::output::DescribeRoleAliasOutput) with field(s):
    ///   - [`role_alias_description(Option<RoleAliasDescription>)`](crate::output::DescribeRoleAliasOutput::role_alias_description): <p>The role alias description.</p>
                            /// - On failure, responds with [`SdkError<DescribeRoleAliasError>`](crate::error::DescribeRoleAliasError)
    pub fn describe_role_alias(&self) -> crate::client::fluent_builders::DescribeRoleAlias {
                                crate::client::fluent_builders::DescribeRoleAlias::new(self.handle.clone())
                            }
}

