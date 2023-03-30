// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Fluent builder constructing a request to `PutEvents`.
/// 
/// <p>Records user interaction event data. For more information see <a href="https://docs.aws.amazon.com/personalize/latest/dg/recording-events.html">Recording Events</a>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct PutEvents {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::put_events_input::Builder
            }
impl PutEvents  {
    /// Creates a new `PutEvents`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::PutEvents, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::PutEventsError>
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
                    pub async fn send(self) -> std::result::Result<crate::output::PutEventsOutput, aws_smithy_http::result::SdkError<crate::error::PutEventsError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// <p>The tracking ID for the event. The ID is generated by a call to the <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_CreateEventTracker.html">CreateEventTracker</a> API.</p>
    pub fn tracking_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.tracking_id(input.into());
        self
    }
    /// <p>The tracking ID for the event. The ID is generated by a call to the <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_CreateEventTracker.html">CreateEventTracker</a> API.</p>
    pub fn set_tracking_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_tracking_id(input);
        self
    }
    /// <p>The user associated with the event.</p>
    pub fn user_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.user_id(input.into());
        self
    }
    /// <p>The user associated with the event.</p>
    pub fn set_user_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_user_id(input);
        self
    }
    /// <p>The session ID associated with the user's visit. Your application generates the sessionId when a user first visits your website or uses your application. Amazon Personalize uses the sessionId to associate events with the user before they log in. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/recording-events.html">Recording Events</a>.</p>
    pub fn session_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.session_id(input.into());
        self
    }
    /// <p>The session ID associated with the user's visit. Your application generates the sessionId when a user first visits your website or uses your application. Amazon Personalize uses the sessionId to associate events with the user before they log in. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/recording-events.html">Recording Events</a>.</p>
    pub fn set_session_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_session_id(input);
        self
    }
    /// Appends an item to `eventList`.
    ///
    /// To override the contents of this collection use [`set_event_list`](Self::set_event_list).
    ///
    /// <p>A list of event data from the session.</p>
    pub fn event_list(mut self, input: crate::model::Event) -> Self {
        self.inner = self.inner.event_list(input);
        self
    }
    /// <p>A list of event data from the session.</p>
    pub fn set_event_list(mut self, input: std::option::Option<std::vec::Vec<crate::model::Event>>) -> Self {
        self.inner = self.inner.set_event_list(input);
        self
    }
}

/// Fluent builder constructing a request to `PutItems`.
/// 
/// <p>Adds one or more items to an Items dataset. For more information see <a href="https://docs.aws.amazon.com/personalize/latest/dg/importing-items.html">Importing Items Incrementally</a>. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct PutItems {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::put_items_input::Builder
            }
impl PutItems  {
    /// Creates a new `PutItems`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::PutItems, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::PutItemsError>
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
                    pub async fn send(self) -> std::result::Result<crate::output::PutItemsOutput, aws_smithy_http::result::SdkError<crate::error::PutItemsError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// <p>The Amazon Resource Name (ARN) of the Items dataset you are adding the item or items to.</p>
    pub fn dataset_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.dataset_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Items dataset you are adding the item or items to.</p>
    pub fn set_dataset_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_dataset_arn(input);
        self
    }
    /// Appends an item to `items`.
    ///
    /// To override the contents of this collection use [`set_items`](Self::set_items).
    ///
    /// <p>A list of item data.</p>
    pub fn items(mut self, input: crate::model::Item) -> Self {
        self.inner = self.inner.items(input);
        self
    }
    /// <p>A list of item data.</p>
    pub fn set_items(mut self, input: std::option::Option<std::vec::Vec<crate::model::Item>>) -> Self {
        self.inner = self.inner.set_items(input);
        self
    }
}

/// Fluent builder constructing a request to `PutUsers`.
/// 
/// <p>Adds one or more users to a Users dataset. For more information see <a href="https://docs.aws.amazon.com/personalize/latest/dg/importing-users.html">Importing Users Incrementally</a>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct PutUsers {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::put_users_input::Builder
            }
impl PutUsers  {
    /// Creates a new `PutUsers`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::PutUsers, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::PutUsersError>
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
                    pub async fn send(self) -> std::result::Result<crate::output::PutUsersOutput, aws_smithy_http::result::SdkError<crate::error::PutUsersError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// <p>The Amazon Resource Name (ARN) of the Users dataset you are adding the user or users to.</p>
    pub fn dataset_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.dataset_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Users dataset you are adding the user or users to.</p>
    pub fn set_dataset_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_dataset_arn(input);
        self
    }
    /// Appends an item to `users`.
    ///
    /// To override the contents of this collection use [`set_users`](Self::set_users).
    ///
    /// <p>A list of user data.</p>
    pub fn users(mut self, input: crate::model::User) -> Self {
        self.inner = self.inner.users(input);
        self
    }
    /// <p>A list of user data.</p>
    pub fn set_users(mut self, input: std::option::Option<std::vec::Vec<crate::model::User>>) -> Self {
        self.inner = self.inner.set_users(input);
        self
    }
}

