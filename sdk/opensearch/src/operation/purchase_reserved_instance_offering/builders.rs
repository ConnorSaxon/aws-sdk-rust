// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::purchase_reserved_instance_offering::_purchase_reserved_instance_offering_output::PurchaseReservedInstanceOfferingOutputBuilder;

pub use crate::operation::purchase_reserved_instance_offering::_purchase_reserved_instance_offering_input::PurchaseReservedInstanceOfferingInputBuilder;

impl PurchaseReservedInstanceOfferingInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::purchase_reserved_instance_offering::PurchaseReservedInstanceOfferingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::purchase_reserved_instance_offering::PurchaseReservedInstanceOfferingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.purchase_reserved_instance_offering();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PurchaseReservedInstanceOffering`.
///
/// <p>Allows you to purchase Amazon OpenSearch Service Reserved Instances.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PurchaseReservedInstanceOfferingFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::purchase_reserved_instance_offering::builders::PurchaseReservedInstanceOfferingInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::purchase_reserved_instance_offering::PurchaseReservedInstanceOfferingOutput,
        crate::operation::purchase_reserved_instance_offering::PurchaseReservedInstanceOfferingError,
    > for PurchaseReservedInstanceOfferingFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::purchase_reserved_instance_offering::PurchaseReservedInstanceOfferingOutput,
            crate::operation::purchase_reserved_instance_offering::PurchaseReservedInstanceOfferingError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PurchaseReservedInstanceOfferingFluentBuilder {
    /// Creates a new `PurchaseReservedInstanceOffering`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PurchaseReservedInstanceOffering as a reference.
    pub fn as_input(&self) -> &crate::operation::purchase_reserved_instance_offering::builders::PurchaseReservedInstanceOfferingInputBuilder {
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
        crate::operation::purchase_reserved_instance_offering::PurchaseReservedInstanceOfferingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::purchase_reserved_instance_offering::PurchaseReservedInstanceOfferingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::purchase_reserved_instance_offering::PurchaseReservedInstanceOffering::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::purchase_reserved_instance_offering::PurchaseReservedInstanceOffering::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::purchase_reserved_instance_offering::PurchaseReservedInstanceOfferingOutput,
        crate::operation::purchase_reserved_instance_offering::PurchaseReservedInstanceOfferingError,
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
    /// <p>The ID of the Reserved Instance offering to purchase.</p>
    pub fn reserved_instance_offering_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.reserved_instance_offering_id(input.into());
        self
    }
    /// <p>The ID of the Reserved Instance offering to purchase.</p>
    pub fn set_reserved_instance_offering_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_reserved_instance_offering_id(input);
        self
    }
    /// <p>The ID of the Reserved Instance offering to purchase.</p>
    pub fn get_reserved_instance_offering_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_reserved_instance_offering_id()
    }
    /// <p>A customer-specified identifier to track this reservation.</p>
    pub fn reservation_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.reservation_name(input.into());
        self
    }
    /// <p>A customer-specified identifier to track this reservation.</p>
    pub fn set_reservation_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_reservation_name(input);
        self
    }
    /// <p>A customer-specified identifier to track this reservation.</p>
    pub fn get_reservation_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_reservation_name()
    }
    /// <p>The number of OpenSearch instances to reserve.</p>
    pub fn instance_count(mut self, input: i32) -> Self {
        self.inner = self.inner.instance_count(input);
        self
    }
    /// <p>The number of OpenSearch instances to reserve.</p>
    pub fn set_instance_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_instance_count(input);
        self
    }
    /// <p>The number of OpenSearch instances to reserve.</p>
    pub fn get_instance_count(&self) -> &::std::option::Option<i32> {
        self.inner.get_instance_count()
    }
}
