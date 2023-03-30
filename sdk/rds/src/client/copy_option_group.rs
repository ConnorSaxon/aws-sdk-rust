// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CopyOptionGroup`](crate::client::fluent_builders::CopyOptionGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`source_option_group_identifier(impl Into<String>)`](crate::client::fluent_builders::CopyOptionGroup::source_option_group_identifier) / [`set_source_option_group_identifier(Option<String>)`](crate::client::fluent_builders::CopyOptionGroup::set_source_option_group_identifier): <p>The identifier for the source option group.</p>  <p>Constraints:</p>  <ul>   <li> <p>Must specify a valid option group.</p> </li>  </ul>
    ///   - [`target_option_group_identifier(impl Into<String>)`](crate::client::fluent_builders::CopyOptionGroup::target_option_group_identifier) / [`set_target_option_group_identifier(Option<String>)`](crate::client::fluent_builders::CopyOptionGroup::set_target_option_group_identifier): <p>The identifier for the copied option group.</p>  <p>Constraints:</p>  <ul>   <li> <p>Can't be null, empty, or blank</p> </li>   <li> <p>Must contain from 1 to 255 letters, numbers, or hyphens</p> </li>   <li> <p>First character must be a letter</p> </li>   <li> <p>Can't end with a hyphen or contain two consecutive hyphens</p> </li>  </ul>  <p>Example: <code>my-option-group</code> </p>
    ///   - [`target_option_group_description(impl Into<String>)`](crate::client::fluent_builders::CopyOptionGroup::target_option_group_description) / [`set_target_option_group_description(Option<String>)`](crate::client::fluent_builders::CopyOptionGroup::set_target_option_group_description): <p>The description for the copied option group.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CopyOptionGroup::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CopyOptionGroup::set_tags): <p>A list of tags. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_Tagging.html">Tagging Amazon RDS Resources</a> in the <i>Amazon RDS User Guide.</i> </p>
                            /// - On success, responds with [`CopyOptionGroupOutput`](crate::output::CopyOptionGroupOutput) with field(s):
    ///   - [`option_group(Option<OptionGroup>)`](crate::output::CopyOptionGroupOutput::option_group): <p></p>
                            /// - On failure, responds with [`SdkError<CopyOptionGroupError>`](crate::error::CopyOptionGroupError)
    pub fn copy_option_group(&self) -> crate::client::fluent_builders::CopyOptionGroup {
                                crate::client::fluent_builders::CopyOptionGroup::new(self.handle.clone())
                            }
}

