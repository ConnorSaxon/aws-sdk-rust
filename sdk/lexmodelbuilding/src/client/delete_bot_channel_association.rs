// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteBotChannelAssociation`](crate::client::fluent_builders::DeleteBotChannelAssociation) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::DeleteBotChannelAssociation::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::DeleteBotChannelAssociation::set_name): <p>The name of the association. The name is case sensitive. </p>
    ///   - [`bot_name(impl Into<String>)`](crate::client::fluent_builders::DeleteBotChannelAssociation::bot_name) / [`set_bot_name(Option<String>)`](crate::client::fluent_builders::DeleteBotChannelAssociation::set_bot_name): <p>The name of the Amazon Lex bot.</p>
    ///   - [`bot_alias(impl Into<String>)`](crate::client::fluent_builders::DeleteBotChannelAssociation::bot_alias) / [`set_bot_alias(Option<String>)`](crate::client::fluent_builders::DeleteBotChannelAssociation::set_bot_alias): <p>An alias that points to the specific version of the Amazon Lex bot to which this association is being made.</p>
                            /// - On success, responds with [`DeleteBotChannelAssociationOutput`](crate::output::DeleteBotChannelAssociationOutput)
                            /// - On failure, responds with [`SdkError<DeleteBotChannelAssociationError>`](crate::error::DeleteBotChannelAssociationError)
    pub fn delete_bot_channel_association(&self) -> crate::client::fluent_builders::DeleteBotChannelAssociation {
                                crate::client::fluent_builders::DeleteBotChannelAssociation::new(self.handle.clone())
                            }
}

