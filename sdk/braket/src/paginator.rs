// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Paginator for [`SearchDevices`](crate::operation::SearchDevices)
            pub struct SearchDevicesPaginator {
                handle: std::sync::Arc<crate::client::Handle>,
                builder: crate::input::search_devices_input::Builder,
                stop_on_duplicate_token: bool,
            }

            impl SearchDevicesPaginator  {
                /// Create a new paginator-wrapper
                pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>, builder: crate::input::search_devices_input::Builder) -> Self {
                    Self {
                        handle,
                        builder,
                        stop_on_duplicate_token: true,
                    }
                }

                /// Set the page size
                ///
                /// _Note: this method will override any previously set value for `max_results`_
                pub fn page_size(mut self, limit: i32) -> Self {
                    self.builder.max_results = Some(limit);
                    self
                }

                /// Create a flattened paginator
                    ///
                    /// This paginator automatically flattens results using `devices`. Queries to the underlying service
                    /// are dispatched lazily.
                    pub fn items(self) -> crate::paginator::SearchDevicesPaginatorItems {
                        crate::paginator::SearchDevicesPaginatorItems(self)
                    }

                /// Stop paginating when the service returns the same pagination token twice in a row.
                ///
                /// Defaults to true.
                ///
                /// For certain operations, it may be useful to continue on duplicate token. For example,
                /// if an operation is for tailing a log file in real-time, then continuing may be desired.
                /// This option can be set to `false` to accommodate these use cases.
                pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {
                    self.stop_on_duplicate_token = stop_on_duplicate_token;
                    self
                }

                /// Create the pagination stream
                ///
                /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
                pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::output::SearchDevicesOutput, aws_smithy_http::result::SdkError<crate::error::SearchDevicesError>>> + Unpin
                 {
                    // Move individual fields out of self for the borrow checker
                    let builder = self.builder;
                    let handle = self.handle;
                    aws_smithy_async::future::fn_stream::FnStream::new(move |tx| Box::pin(async move {
                        // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                        let mut input = match builder.build().map_err(aws_smithy_http::result::SdkError::construction_failure) {
                            Ok(input) => input,
                            Err(e) => { let _ = tx.send(Err(e)).await; return; }
                        };
                        loop {
                            let op = match input.make_operation(&handle.conf)
                                .await
                                .map_err(aws_smithy_http::result::SdkError::construction_failure) {
                                Ok(op) => op,
                                Err(e) => {
                                    let _ = tx.send(Err(e)).await;
                                    return;
                                }
                            };
                            let resp = handle.client.call(op).await;
                            // If the input member is None or it was an error
                            let done = match resp {
                                Ok(ref resp) => {
                                    let new_token = crate::lens::reflens_search_devices_output_next_token(resp);
                                    let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                                    if !is_empty && new_token == input.next_token.as_ref() && self.stop_on_duplicate_token {
                                        true
                                    } else {
                                        input.next_token = new_token.cloned();
                                        is_empty
                                    }
                                },
                                Err(_) => true,
                            };
                            if tx.send(resp).await.is_err() {
                                // receiving end was dropped
                                return
                            }
                            if done {
                                return
                            }
                        }
                    }))
                }
            }

/// Paginator for [`SearchJobs`](crate::operation::SearchJobs)
            pub struct SearchJobsPaginator {
                handle: std::sync::Arc<crate::client::Handle>,
                builder: crate::input::search_jobs_input::Builder,
                stop_on_duplicate_token: bool,
            }

            impl SearchJobsPaginator  {
                /// Create a new paginator-wrapper
                pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>, builder: crate::input::search_jobs_input::Builder) -> Self {
                    Self {
                        handle,
                        builder,
                        stop_on_duplicate_token: true,
                    }
                }

                /// Set the page size
                ///
                /// _Note: this method will override any previously set value for `max_results`_
                pub fn page_size(mut self, limit: i32) -> Self {
                    self.builder.max_results = Some(limit);
                    self
                }

                /// Create a flattened paginator
                    ///
                    /// This paginator automatically flattens results using `jobs`. Queries to the underlying service
                    /// are dispatched lazily.
                    pub fn items(self) -> crate::paginator::SearchJobsPaginatorItems {
                        crate::paginator::SearchJobsPaginatorItems(self)
                    }

                /// Stop paginating when the service returns the same pagination token twice in a row.
                ///
                /// Defaults to true.
                ///
                /// For certain operations, it may be useful to continue on duplicate token. For example,
                /// if an operation is for tailing a log file in real-time, then continuing may be desired.
                /// This option can be set to `false` to accommodate these use cases.
                pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {
                    self.stop_on_duplicate_token = stop_on_duplicate_token;
                    self
                }

                /// Create the pagination stream
                ///
                /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
                pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::output::SearchJobsOutput, aws_smithy_http::result::SdkError<crate::error::SearchJobsError>>> + Unpin
                 {
                    // Move individual fields out of self for the borrow checker
                    let builder = self.builder;
                    let handle = self.handle;
                    aws_smithy_async::future::fn_stream::FnStream::new(move |tx| Box::pin(async move {
                        // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                        let mut input = match builder.build().map_err(aws_smithy_http::result::SdkError::construction_failure) {
                            Ok(input) => input,
                            Err(e) => { let _ = tx.send(Err(e)).await; return; }
                        };
                        loop {
                            let op = match input.make_operation(&handle.conf)
                                .await
                                .map_err(aws_smithy_http::result::SdkError::construction_failure) {
                                Ok(op) => op,
                                Err(e) => {
                                    let _ = tx.send(Err(e)).await;
                                    return;
                                }
                            };
                            let resp = handle.client.call(op).await;
                            // If the input member is None or it was an error
                            let done = match resp {
                                Ok(ref resp) => {
                                    let new_token = crate::lens::reflens_search_jobs_output_next_token(resp);
                                    let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                                    if !is_empty && new_token == input.next_token.as_ref() && self.stop_on_duplicate_token {
                                        true
                                    } else {
                                        input.next_token = new_token.cloned();
                                        is_empty
                                    }
                                },
                                Err(_) => true,
                            };
                            if tx.send(resp).await.is_err() {
                                // receiving end was dropped
                                return
                            }
                            if done {
                                return
                            }
                        }
                    }))
                }
            }

/// Paginator for [`SearchQuantumTasks`](crate::operation::SearchQuantumTasks)
            pub struct SearchQuantumTasksPaginator {
                handle: std::sync::Arc<crate::client::Handle>,
                builder: crate::input::search_quantum_tasks_input::Builder,
                stop_on_duplicate_token: bool,
            }

            impl SearchQuantumTasksPaginator  {
                /// Create a new paginator-wrapper
                pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>, builder: crate::input::search_quantum_tasks_input::Builder) -> Self {
                    Self {
                        handle,
                        builder,
                        stop_on_duplicate_token: true,
                    }
                }

                /// Set the page size
                ///
                /// _Note: this method will override any previously set value for `max_results`_
                pub fn page_size(mut self, limit: i32) -> Self {
                    self.builder.max_results = Some(limit);
                    self
                }

                /// Create a flattened paginator
                    ///
                    /// This paginator automatically flattens results using `quantum_tasks`. Queries to the underlying service
                    /// are dispatched lazily.
                    pub fn items(self) -> crate::paginator::SearchQuantumTasksPaginatorItems {
                        crate::paginator::SearchQuantumTasksPaginatorItems(self)
                    }

                /// Stop paginating when the service returns the same pagination token twice in a row.
                ///
                /// Defaults to true.
                ///
                /// For certain operations, it may be useful to continue on duplicate token. For example,
                /// if an operation is for tailing a log file in real-time, then continuing may be desired.
                /// This option can be set to `false` to accommodate these use cases.
                pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {
                    self.stop_on_duplicate_token = stop_on_duplicate_token;
                    self
                }

                /// Create the pagination stream
                ///
                /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
                pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::output::SearchQuantumTasksOutput, aws_smithy_http::result::SdkError<crate::error::SearchQuantumTasksError>>> + Unpin
                 {
                    // Move individual fields out of self for the borrow checker
                    let builder = self.builder;
                    let handle = self.handle;
                    aws_smithy_async::future::fn_stream::FnStream::new(move |tx| Box::pin(async move {
                        // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                        let mut input = match builder.build().map_err(aws_smithy_http::result::SdkError::construction_failure) {
                            Ok(input) => input,
                            Err(e) => { let _ = tx.send(Err(e)).await; return; }
                        };
                        loop {
                            let op = match input.make_operation(&handle.conf)
                                .await
                                .map_err(aws_smithy_http::result::SdkError::construction_failure) {
                                Ok(op) => op,
                                Err(e) => {
                                    let _ = tx.send(Err(e)).await;
                                    return;
                                }
                            };
                            let resp = handle.client.call(op).await;
                            // If the input member is None or it was an error
                            let done = match resp {
                                Ok(ref resp) => {
                                    let new_token = crate::lens::reflens_search_quantum_tasks_output_next_token(resp);
                                    let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                                    if !is_empty && new_token == input.next_token.as_ref() && self.stop_on_duplicate_token {
                                        true
                                    } else {
                                        input.next_token = new_token.cloned();
                                        is_empty
                                    }
                                },
                                Err(_) => true,
                            };
                            if tx.send(resp).await.is_err() {
                                // receiving end was dropped
                                return
                            }
                            if done {
                                return
                            }
                        }
                    }))
                }
            }

/// Flattened paginator for `SearchDevicesPaginator`
                ///
                /// This is created with [`.items()`](SearchDevicesPaginator::items)
                pub struct SearchDevicesPaginatorItems(SearchDevicesPaginator);

                impl  SearchDevicesPaginatorItems  {
                    /// Create the pagination stream
                    ///
                    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
                    ///
                    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
                    pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::model::DeviceSummary, aws_smithy_http::result::SdkError<crate::error::SearchDevicesError>>> + Unpin
                     {
                        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_search_devices_output_devices(page).unwrap_or_default().into_iter())
                    }
                }

/// Flattened paginator for `SearchJobsPaginator`
                ///
                /// This is created with [`.items()`](SearchJobsPaginator::items)
                pub struct SearchJobsPaginatorItems(SearchJobsPaginator);

                impl  SearchJobsPaginatorItems  {
                    /// Create the pagination stream
                    ///
                    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
                    ///
                    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
                    pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::model::JobSummary, aws_smithy_http::result::SdkError<crate::error::SearchJobsError>>> + Unpin
                     {
                        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_search_jobs_output_jobs(page).unwrap_or_default().into_iter())
                    }
                }

/// Flattened paginator for `SearchQuantumTasksPaginator`
                ///
                /// This is created with [`.items()`](SearchQuantumTasksPaginator::items)
                pub struct SearchQuantumTasksPaginatorItems(SearchQuantumTasksPaginator);

                impl  SearchQuantumTasksPaginatorItems  {
                    /// Create the pagination stream
                    ///
                    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
                    ///
                    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
                    pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::model::QuantumTaskSummary, aws_smithy_http::result::SdkError<crate::error::SearchQuantumTasksError>>> + Unpin
                     {
                        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_search_quantum_tasks_output_quantum_tasks(page).unwrap_or_default().into_iter())
                    }
                }

