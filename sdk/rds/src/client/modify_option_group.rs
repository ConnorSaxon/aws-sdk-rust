// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ModifyOptionGroup`](crate::client::fluent_builders::ModifyOptionGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`option_group_name(impl Into<String>)`](crate::client::fluent_builders::ModifyOptionGroup::option_group_name) / [`set_option_group_name(Option<String>)`](crate::client::fluent_builders::ModifyOptionGroup::set_option_group_name): <p>The name of the option group to be modified.</p>  <p>Permanent options, such as the TDE option for Oracle Advanced Security TDE, can't be removed from an option group, and that option group can't be removed from a DB instance once it is associated with a DB instance</p>
    ///   - [`options_to_include(Vec<OptionConfiguration>)`](crate::client::fluent_builders::ModifyOptionGroup::options_to_include) / [`set_options_to_include(Option<Vec<OptionConfiguration>>)`](crate::client::fluent_builders::ModifyOptionGroup::set_options_to_include): <p>Options in this list are added to the option group or, if already present, the specified configuration is used to update the existing configuration.</p>
    ///   - [`options_to_remove(Vec<String>)`](crate::client::fluent_builders::ModifyOptionGroup::options_to_remove) / [`set_options_to_remove(Option<Vec<String>>)`](crate::client::fluent_builders::ModifyOptionGroup::set_options_to_remove): <p>Options in this list are removed from the option group.</p>
    ///   - [`apply_immediately(bool)`](crate::client::fluent_builders::ModifyOptionGroup::apply_immediately) / [`set_apply_immediately(bool)`](crate::client::fluent_builders::ModifyOptionGroup::set_apply_immediately): <p>A value that indicates whether to apply the change immediately or during the next maintenance window for each instance associated with the option group.</p>
                            /// - On success, responds with [`ModifyOptionGroupOutput`](crate::output::ModifyOptionGroupOutput) with field(s):
    ///   - [`option_group(Option<OptionGroup>)`](crate::output::ModifyOptionGroupOutput::option_group): <p></p>
                            /// - On failure, responds with [`SdkError<ModifyOptionGroupError>`](crate::error::ModifyOptionGroupError)
    pub fn modify_option_group(&self) -> crate::client::fluent_builders::ModifyOptionGroup {
                                crate::client::fluent_builders::ModifyOptionGroup::new(self.handle.clone())
                            }
}

