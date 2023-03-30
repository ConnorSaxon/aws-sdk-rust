// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Fluent builder constructing a request to `DeleteThingShadow`.
/// 
/// <p>Deletes the shadow for the specified thing.</p> 
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">DeleteThingShadow</a> action.</p> 
/// <p>For more information, see <a href="http://docs.aws.amazon.com/iot/latest/developerguide/API_DeleteThingShadow.html">DeleteThingShadow</a> in the IoT Developer Guide.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteThingShadow {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::delete_thing_shadow_input::Builder
            }
impl DeleteThingShadow  {
    /// Creates a new `DeleteThingShadow`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::DeleteThingShadow, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::DeleteThingShadowError>
                    >  {
                        let handle = self.handle.clone();
                        let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                    }
    
                    /// Sends the request and returns the response.
                    ///
                    /// If an error occurs, an `SdkError` will be returned with additional details that
                    /// can be matched against.
                    ///
                    /// By default, any retryable failures will be retried twice. Retry behavior
                    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::output::DeleteThingShadowOutput, aws_smithy_http::result::SdkError<crate::error::DeleteThingShadowError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// <p>The name of the thing.</p>
    pub fn thing_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.thing_name(input.into());
        self
    }
    /// <p>The name of the thing.</p>
    pub fn set_thing_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_thing_name(input);
        self
    }
    /// <p>The name of the shadow.</p>
    pub fn shadow_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.shadow_name(input.into());
        self
    }
    /// <p>The name of the shadow.</p>
    pub fn set_shadow_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_shadow_name(input);
        self
    }
}

/// Fluent builder constructing a request to `GetRetainedMessage`.
/// 
/// <p>Gets the details of a single retained message for the specified topic.</p> 
/// <p>This action returns the message payload of the retained message, which can incur messaging costs. To list only the topic names of the retained messages, call <a href="/iot/latest/developerguide/API_iotdata_ListRetainedMessages.html">ListRetainedMessages</a>.</p> 
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiotfleethubfordevicemanagement.html#awsiotfleethubfordevicemanagement-actions-as-permissions">GetRetainedMessage</a> action.</p> 
/// <p>For more information about messaging costs, see <a href="http://aws.amazon.com/iot-core/pricing/#Messaging">Amazon Web Services IoT Core pricing - Messaging</a>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetRetainedMessage {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::get_retained_message_input::Builder
            }
impl GetRetainedMessage  {
    /// Creates a new `GetRetainedMessage`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::GetRetainedMessage, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::GetRetainedMessageError>
                    >  {
                        let handle = self.handle.clone();
                        let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                    }
    
                    /// Sends the request and returns the response.
                    ///
                    /// If an error occurs, an `SdkError` will be returned with additional details that
                    /// can be matched against.
                    ///
                    /// By default, any retryable failures will be retried twice. Retry behavior
                    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::output::GetRetainedMessageOutput, aws_smithy_http::result::SdkError<crate::error::GetRetainedMessageError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// <p>The topic name of the retained message to retrieve.</p>
    pub fn topic(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.topic(input.into());
        self
    }
    /// <p>The topic name of the retained message to retrieve.</p>
    pub fn set_topic(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_topic(input);
        self
    }
}

/// Fluent builder constructing a request to `GetThingShadow`.
/// 
/// <p>Gets the shadow for the specified thing.</p> 
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">GetThingShadow</a> action.</p> 
/// <p>For more information, see <a href="http://docs.aws.amazon.com/iot/latest/developerguide/API_GetThingShadow.html">GetThingShadow</a> in the IoT Developer Guide.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetThingShadow {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::get_thing_shadow_input::Builder
            }
impl GetThingShadow  {
    /// Creates a new `GetThingShadow`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::GetThingShadow, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::GetThingShadowError>
                    >  {
                        let handle = self.handle.clone();
                        let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                    }
    
                    /// Sends the request and returns the response.
                    ///
                    /// If an error occurs, an `SdkError` will be returned with additional details that
                    /// can be matched against.
                    ///
                    /// By default, any retryable failures will be retried twice. Retry behavior
                    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::output::GetThingShadowOutput, aws_smithy_http::result::SdkError<crate::error::GetThingShadowError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// <p>The name of the thing.</p>
    pub fn thing_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.thing_name(input.into());
        self
    }
    /// <p>The name of the thing.</p>
    pub fn set_thing_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_thing_name(input);
        self
    }
    /// <p>The name of the shadow.</p>
    pub fn shadow_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.shadow_name(input.into());
        self
    }
    /// <p>The name of the shadow.</p>
    pub fn set_shadow_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_shadow_name(input);
        self
    }
}

/// Fluent builder constructing a request to `ListNamedShadowsForThing`.
/// 
/// <p>Lists the shadows for the specified thing.</p> 
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">ListNamedShadowsForThing</a> action.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListNamedShadowsForThing {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::list_named_shadows_for_thing_input::Builder
            }
impl ListNamedShadowsForThing  {
    /// Creates a new `ListNamedShadowsForThing`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::ListNamedShadowsForThing, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::ListNamedShadowsForThingError>
                    >  {
                        let handle = self.handle.clone();
                        let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                    }
    
                    /// Sends the request and returns the response.
                    ///
                    /// If an error occurs, an `SdkError` will be returned with additional details that
                    /// can be matched against.
                    ///
                    /// By default, any retryable failures will be retried twice. Retry behavior
                    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::output::ListNamedShadowsForThingOutput, aws_smithy_http::result::SdkError<crate::error::ListNamedShadowsForThingError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// <p>The name of the thing.</p>
    pub fn thing_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.thing_name(input.into());
        self
    }
    /// <p>The name of the thing.</p>
    pub fn set_thing_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_thing_name(input);
        self
    }
    /// <p>The token to retrieve the next set of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token to retrieve the next set of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The result page size.</p>
    pub fn page_size(mut self, input: i32) -> Self {
        self.inner = self.inner.page_size(input);
        self
    }
    /// <p>The result page size.</p>
    pub fn set_page_size(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_page_size(input);
        self
    }
}

/// Fluent builder constructing a request to `ListRetainedMessages`.
/// 
/// <p>Lists summary information about the retained messages stored for the account.</p> 
/// <p>This action returns only the topic names of the retained messages. It doesn't return any message payloads. Although this action doesn't return a message payload, it can still incur messaging costs.</p> 
/// <p>To get the message payload of a retained message, call <a href="https://docs.aws.amazon.com/iot/latest/developerguide/API_iotdata_GetRetainedMessage.html">GetRetainedMessage</a> with the topic name of the retained message.</p> 
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiotfleethubfordevicemanagement.html#awsiotfleethubfordevicemanagement-actions-as-permissions">ListRetainedMessages</a> action.</p> 
/// <p>For more information about messaging costs, see <a href="http://aws.amazon.com/iot-core/pricing/#Messaging">Amazon Web Services IoT Core pricing - Messaging</a>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListRetainedMessages {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::list_retained_messages_input::Builder
            }
impl ListRetainedMessages  {
    /// Creates a new `ListRetainedMessages`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::ListRetainedMessages, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::ListRetainedMessagesError>
                    >  {
                        let handle = self.handle.clone();
                        let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                    }
    
                    /// Sends the request and returns the response.
                    ///
                    /// If an error occurs, an `SdkError` will be returned with additional details that
                    /// can be matched against.
                    ///
                    /// By default, any retryable failures will be retried twice. Retry behavior
                    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::output::ListRetainedMessagesOutput, aws_smithy_http::result::SdkError<crate::error::ListRetainedMessagesError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// Create a paginator for this request
                        ///
                        /// Paginators are used by calling [`send().await`](crate::paginator::ListRetainedMessagesPaginator::send) which returns a `Stream`.
                        pub fn into_paginator(self) -> crate::paginator::ListRetainedMessagesPaginator {
                            crate::paginator::ListRetainedMessagesPaginator::new(self.handle, self.inner)
                        }
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of results to return at one time.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return at one time.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}

/// Fluent builder constructing a request to `Publish`.
/// 
/// <p>Publishes an MQTT message.</p> 
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">Publish</a> action.</p> 
/// <p>For more information about MQTT messages, see <a href="http://docs.aws.amazon.com/iot/latest/developerguide/mqtt.html">MQTT Protocol</a> in the IoT Developer Guide.</p> 
/// <p>For more information about messaging costs, see <a href="http://aws.amazon.com/iot-core/pricing/#Messaging">Amazon Web Services IoT Core pricing - Messaging</a>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct Publish {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::publish_input::Builder
            }
impl Publish  {
    /// Creates a new `Publish`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::Publish, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::PublishError>
                    >  {
                        let handle = self.handle.clone();
                        let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                    }
    
                    /// Sends the request and returns the response.
                    ///
                    /// If an error occurs, an `SdkError` will be returned with additional details that
                    /// can be matched against.
                    ///
                    /// By default, any retryable failures will be retried twice. Retry behavior
                    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::output::PublishOutput, aws_smithy_http::result::SdkError<crate::error::PublishError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// <p>The name of the MQTT topic.</p>
    pub fn topic(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.topic(input.into());
        self
    }
    /// <p>The name of the MQTT topic.</p>
    pub fn set_topic(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_topic(input);
        self
    }
    /// <p>The Quality of Service (QoS) level. The default QoS level is 0.</p>
    pub fn qos(mut self, input: i32) -> Self {
        self.inner = self.inner.qos(input);
        self
    }
    /// <p>The Quality of Service (QoS) level. The default QoS level is 0.</p>
    pub fn set_qos(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_qos(input);
        self
    }
    /// <p>A Boolean value that determines whether to set the RETAIN flag when the message is published.</p> 
    /// <p>Setting the RETAIN flag causes the message to be retained and sent to new subscribers to the topic.</p> 
    /// <p>Valid values: <code>true</code> | <code>false</code> </p> 
    /// <p>Default value: <code>false</code> </p>
    pub fn retain(mut self, input: bool) -> Self {
        self.inner = self.inner.retain(input);
        self
    }
    /// <p>A Boolean value that determines whether to set the RETAIN flag when the message is published.</p> 
    /// <p>Setting the RETAIN flag causes the message to be retained and sent to new subscribers to the topic.</p> 
    /// <p>Valid values: <code>true</code> | <code>false</code> </p> 
    /// <p>Default value: <code>false</code> </p>
    pub fn set_retain(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_retain(input);
        self
    }
    /// <p>The message body. MQTT accepts text, binary, and empty (null) message payloads.</p> 
    /// <p>Publishing an empty (null) payload with <b>retain</b> = <code>true</code> deletes the retained message identified by <b>topic</b> from Amazon Web Services IoT Core.</p>
    pub fn payload(mut self, input: aws_smithy_types::Blob) -> Self {
        self.inner = self.inner.payload(input);
        self
    }
    /// <p>The message body. MQTT accepts text, binary, and empty (null) message payloads.</p> 
    /// <p>Publishing an empty (null) payload with <b>retain</b> = <code>true</code> deletes the retained message identified by <b>topic</b> from Amazon Web Services IoT Core.</p>
    pub fn set_payload(mut self, input: std::option::Option<aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_payload(input);
        self
    }
    /// <p>A JSON string that contains an array of JSON objects. If you don’t use Amazon Web Services SDK or CLI, you must encode the JSON string to base64 format before adding it to the HTTP header. <code>userProperties</code> is an HTTP header value in the API.</p> 
    /// <p>The following example <code>userProperties</code> parameter is a JSON string which represents two User Properties. Note that it needs to be base64-encoded:</p> 
    /// <p> <code>[{"deviceName": "alpha"}, {"deviceCnt": "45"}]</code> </p>
    pub fn user_properties(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.user_properties(input.into());
        self
    }
    /// <p>A JSON string that contains an array of JSON objects. If you don’t use Amazon Web Services SDK or CLI, you must encode the JSON string to base64 format before adding it to the HTTP header. <code>userProperties</code> is an HTTP header value in the API.</p> 
    /// <p>The following example <code>userProperties</code> parameter is a JSON string which represents two User Properties. Note that it needs to be base64-encoded:</p> 
    /// <p> <code>[{"deviceName": "alpha"}, {"deviceCnt": "45"}]</code> </p>
    pub fn set_user_properties(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_user_properties(input);
        self
    }
    /// <p>An <code>Enum</code> string value that indicates whether the payload is formatted as UTF-8. <code>payloadFormatIndicator</code> is an HTTP header value in the API.</p>
    pub fn payload_format_indicator(mut self, input: crate::model::PayloadFormatIndicator) -> Self {
        self.inner = self.inner.payload_format_indicator(input);
        self
    }
    /// <p>An <code>Enum</code> string value that indicates whether the payload is formatted as UTF-8. <code>payloadFormatIndicator</code> is an HTTP header value in the API.</p>
    pub fn set_payload_format_indicator(mut self, input: std::option::Option<crate::model::PayloadFormatIndicator>) -> Self {
        self.inner = self.inner.set_payload_format_indicator(input);
        self
    }
    /// <p>A UTF-8 encoded string that describes the content of the publishing message.</p>
    pub fn content_type(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.content_type(input.into());
        self
    }
    /// <p>A UTF-8 encoded string that describes the content of the publishing message.</p>
    pub fn set_content_type(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_content_type(input);
        self
    }
    /// <p>A UTF-8 encoded string that's used as the topic name for a response message. The response topic is used to describe the topic which the receiver should publish to as part of the request-response flow. The topic must not contain wildcard characters.</p>
    pub fn response_topic(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.response_topic(input.into());
        self
    }
    /// <p>A UTF-8 encoded string that's used as the topic name for a response message. The response topic is used to describe the topic which the receiver should publish to as part of the request-response flow. The topic must not contain wildcard characters.</p>
    pub fn set_response_topic(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_response_topic(input);
        self
    }
    /// <p>The base64-encoded binary data used by the sender of the request message to identify which request the response message is for when it's received. <code>correlationData</code> is an HTTP header value in the API.</p>
    pub fn correlation_data(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.correlation_data(input.into());
        self
    }
    /// <p>The base64-encoded binary data used by the sender of the request message to identify which request the response message is for when it's received. <code>correlationData</code> is an HTTP header value in the API.</p>
    pub fn set_correlation_data(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_correlation_data(input);
        self
    }
    /// <p>A user-defined integer value that represents the message expiry interval in seconds. If absent, the message doesn't expire. For more information about the limits of <code>messageExpiry</code>, see <a href="https://docs.aws.amazon.com/general/latest/gr/iot-core.html#message-broker-limits">Amazon Web Services IoT Core message broker and protocol limits and quotas </a> from the Amazon Web Services Reference Guide.</p>
    pub fn message_expiry(mut self, input: i64) -> Self {
        self.inner = self.inner.message_expiry(input);
        self
    }
    /// <p>A user-defined integer value that represents the message expiry interval in seconds. If absent, the message doesn't expire. For more information about the limits of <code>messageExpiry</code>, see <a href="https://docs.aws.amazon.com/general/latest/gr/iot-core.html#message-broker-limits">Amazon Web Services IoT Core message broker and protocol limits and quotas </a> from the Amazon Web Services Reference Guide.</p>
    pub fn set_message_expiry(mut self, input: std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_message_expiry(input);
        self
    }
}

/// Fluent builder constructing a request to `UpdateThingShadow`.
/// 
/// <p>Updates the shadow for the specified thing.</p> 
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">UpdateThingShadow</a> action.</p> 
/// <p>For more information, see <a href="http://docs.aws.amazon.com/iot/latest/developerguide/API_UpdateThingShadow.html">UpdateThingShadow</a> in the IoT Developer Guide.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateThingShadow {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::update_thing_shadow_input::Builder
            }
impl UpdateThingShadow  {
    /// Creates a new `UpdateThingShadow`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::UpdateThingShadow, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::UpdateThingShadowError>
                    >  {
                        let handle = self.handle.clone();
                        let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                    }
    
                    /// Sends the request and returns the response.
                    ///
                    /// If an error occurs, an `SdkError` will be returned with additional details that
                    /// can be matched against.
                    ///
                    /// By default, any retryable failures will be retried twice. Retry behavior
                    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::output::UpdateThingShadowOutput, aws_smithy_http::result::SdkError<crate::error::UpdateThingShadowError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// <p>The name of the thing.</p>
    pub fn thing_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.thing_name(input.into());
        self
    }
    /// <p>The name of the thing.</p>
    pub fn set_thing_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_thing_name(input);
        self
    }
    /// <p>The name of the shadow.</p>
    pub fn shadow_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.shadow_name(input.into());
        self
    }
    /// <p>The name of the shadow.</p>
    pub fn set_shadow_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_shadow_name(input);
        self
    }
    /// <p>The state information, in JSON format.</p>
    pub fn payload(mut self, input: aws_smithy_types::Blob) -> Self {
        self.inner = self.inner.payload(input);
        self
    }
    /// <p>The state information, in JSON format.</p>
    pub fn set_payload(mut self, input: std::option::Option<aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_payload(input);
        self
    }
}

