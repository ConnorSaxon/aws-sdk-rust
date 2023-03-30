// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeIndexFields`](crate::client::fluent_builders::DescribeIndexFields) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`domain_name(impl Into<String>)`](crate::client::fluent_builders::DescribeIndexFields::domain_name) / [`set_domain_name(Option<String>)`](crate::client::fluent_builders::DescribeIndexFields::set_domain_name): <p>The name of the domain you want to describe.</p>
    ///   - [`field_names(Vec<String>)`](crate::client::fluent_builders::DescribeIndexFields::field_names) / [`set_field_names(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeIndexFields::set_field_names): <p>A list of the index fields you want to describe. If not specified, information is returned for all configured index fields.</p>
    ///   - [`deployed(bool)`](crate::client::fluent_builders::DescribeIndexFields::deployed) / [`set_deployed(Option<bool>)`](crate::client::fluent_builders::DescribeIndexFields::set_deployed): <p>Whether to display the deployed configuration (<code>true</code>) or include any pending changes (<code>false</code>). Defaults to <code>false</code>.</p>
                            /// - On success, responds with [`DescribeIndexFieldsOutput`](crate::output::DescribeIndexFieldsOutput) with field(s):
    ///   - [`index_fields(Option<Vec<IndexFieldStatus>>)`](crate::output::DescribeIndexFieldsOutput::index_fields): <p>The index fields configured for the domain.</p>
                            /// - On failure, responds with [`SdkError<DescribeIndexFieldsError>`](crate::error::DescribeIndexFieldsError)
    pub fn describe_index_fields(&self) -> crate::client::fluent_builders::DescribeIndexFields {
                                crate::client::fluent_builders::DescribeIndexFields::new(self.handle.clone())
                            }
}

