// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_sender_id::_update_sender_id_output::UpdateSenderIdOutputBuilder;

pub use crate::operation::update_sender_id::_update_sender_id_input::UpdateSenderIdInputBuilder;

impl UpdateSenderIdInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_sender_id::UpdateSenderIdOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_sender_id::UpdateSenderIdError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_sender_id();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateSenderId`.
///
/// <p>Updates the configuration of an existing sender ID.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateSenderIdFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_sender_id::builders::UpdateSenderIdInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_sender_id::UpdateSenderIdOutput,
        crate::operation::update_sender_id::UpdateSenderIdError,
    > for UpdateSenderIdFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_sender_id::UpdateSenderIdOutput,
            crate::operation::update_sender_id::UpdateSenderIdError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateSenderIdFluentBuilder {
    /// Creates a new `UpdateSenderId`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateSenderId as a reference.
    pub fn as_input(&self) -> &crate::operation::update_sender_id::builders::UpdateSenderIdInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_sender_id::UpdateSenderIdOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_sender_id::UpdateSenderIdError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_sender_id::UpdateSenderId::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_sender_id::UpdateSenderId::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_sender_id::UpdateSenderIdOutput,
        crate::operation::update_sender_id::UpdateSenderIdError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The sender ID to update.</p>
    pub fn sender_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.sender_id(input.into());
        self
    }
    /// <p>The sender ID to update.</p>
    pub fn set_sender_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_sender_id(input);
        self
    }
    /// <p>The sender ID to update.</p>
    pub fn get_sender_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_sender_id()
    }
    /// <p>The two-character code, in ISO 3166-1 alpha-2 format, for the country or region.</p>
    pub fn iso_country_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.iso_country_code(input.into());
        self
    }
    /// <p>The two-character code, in ISO 3166-1 alpha-2 format, for the country or region.</p>
    pub fn set_iso_country_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_iso_country_code(input);
        self
    }
    /// <p>The two-character code, in ISO 3166-1 alpha-2 format, for the country or region.</p>
    pub fn get_iso_country_code(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_iso_country_code()
    }
    /// <p>By default this is set to false. When set to true the sender ID can't be deleted.</p>
    pub fn deletion_protection_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.deletion_protection_enabled(input);
        self
    }
    /// <p>By default this is set to false. When set to true the sender ID can't be deleted.</p>
    pub fn set_deletion_protection_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_deletion_protection_enabled(input);
        self
    }
    /// <p>By default this is set to false. When set to true the sender ID can't be deleted.</p>
    pub fn get_deletion_protection_enabled(&self) -> &::std::option::Option<bool> {
        self.inner.get_deletion_protection_enabled()
    }
}
