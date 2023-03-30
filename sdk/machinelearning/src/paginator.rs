// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Paginator for [`DescribeBatchPredictions`](crate::operation::DescribeBatchPredictions)
            pub struct DescribeBatchPredictionsPaginator {
                handle: std::sync::Arc<crate::client::Handle>,
                builder: crate::input::describe_batch_predictions_input::Builder,
                stop_on_duplicate_token: bool,
            }

            impl DescribeBatchPredictionsPaginator  {
                /// Create a new paginator-wrapper
                pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>, builder: crate::input::describe_batch_predictions_input::Builder) -> Self {
                    Self {
                        handle,
                        builder,
                        stop_on_duplicate_token: true,
                    }
                }

                /// Set the page size
                ///
                /// _Note: this method will override any previously set value for `limit`_
                pub fn page_size(mut self, limit: i32) -> Self {
                    self.builder.limit = Some(limit);
                    self
                }

                /// Create a flattened paginator
                    ///
                    /// This paginator automatically flattens results using `results`. Queries to the underlying service
                    /// are dispatched lazily.
                    pub fn items(self) -> crate::paginator::DescribeBatchPredictionsPaginatorItems {
                        crate::paginator::DescribeBatchPredictionsPaginatorItems(self)
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
                pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::output::DescribeBatchPredictionsOutput, aws_smithy_http::result::SdkError<crate::error::DescribeBatchPredictionsError>>> + Unpin
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
                                    let new_token = crate::lens::reflens_describe_batch_predictions_output_next_token(resp);
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

/// Paginator for [`DescribeDataSources`](crate::operation::DescribeDataSources)
            pub struct DescribeDataSourcesPaginator {
                handle: std::sync::Arc<crate::client::Handle>,
                builder: crate::input::describe_data_sources_input::Builder,
                stop_on_duplicate_token: bool,
            }

            impl DescribeDataSourcesPaginator  {
                /// Create a new paginator-wrapper
                pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>, builder: crate::input::describe_data_sources_input::Builder) -> Self {
                    Self {
                        handle,
                        builder,
                        stop_on_duplicate_token: true,
                    }
                }

                /// Set the page size
                ///
                /// _Note: this method will override any previously set value for `limit`_
                pub fn page_size(mut self, limit: i32) -> Self {
                    self.builder.limit = Some(limit);
                    self
                }

                /// Create a flattened paginator
                    ///
                    /// This paginator automatically flattens results using `results`. Queries to the underlying service
                    /// are dispatched lazily.
                    pub fn items(self) -> crate::paginator::DescribeDataSourcesPaginatorItems {
                        crate::paginator::DescribeDataSourcesPaginatorItems(self)
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
                pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::output::DescribeDataSourcesOutput, aws_smithy_http::result::SdkError<crate::error::DescribeDataSourcesError>>> + Unpin
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
                                    let new_token = crate::lens::reflens_describe_data_sources_output_next_token(resp);
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

/// Paginator for [`DescribeEvaluations`](crate::operation::DescribeEvaluations)
            pub struct DescribeEvaluationsPaginator {
                handle: std::sync::Arc<crate::client::Handle>,
                builder: crate::input::describe_evaluations_input::Builder,
                stop_on_duplicate_token: bool,
            }

            impl DescribeEvaluationsPaginator  {
                /// Create a new paginator-wrapper
                pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>, builder: crate::input::describe_evaluations_input::Builder) -> Self {
                    Self {
                        handle,
                        builder,
                        stop_on_duplicate_token: true,
                    }
                }

                /// Set the page size
                ///
                /// _Note: this method will override any previously set value for `limit`_
                pub fn page_size(mut self, limit: i32) -> Self {
                    self.builder.limit = Some(limit);
                    self
                }

                /// Create a flattened paginator
                    ///
                    /// This paginator automatically flattens results using `results`. Queries to the underlying service
                    /// are dispatched lazily.
                    pub fn items(self) -> crate::paginator::DescribeEvaluationsPaginatorItems {
                        crate::paginator::DescribeEvaluationsPaginatorItems(self)
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
                pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::output::DescribeEvaluationsOutput, aws_smithy_http::result::SdkError<crate::error::DescribeEvaluationsError>>> + Unpin
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
                                    let new_token = crate::lens::reflens_describe_evaluations_output_next_token(resp);
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

/// Paginator for [`DescribeMLModels`](crate::operation::DescribeMLModels)
            pub struct DescribeMlModelsPaginator {
                handle: std::sync::Arc<crate::client::Handle>,
                builder: crate::input::describe_ml_models_input::Builder,
                stop_on_duplicate_token: bool,
            }

            impl DescribeMlModelsPaginator  {
                /// Create a new paginator-wrapper
                pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>, builder: crate::input::describe_ml_models_input::Builder) -> Self {
                    Self {
                        handle,
                        builder,
                        stop_on_duplicate_token: true,
                    }
                }

                /// Set the page size
                ///
                /// _Note: this method will override any previously set value for `limit`_
                pub fn page_size(mut self, limit: i32) -> Self {
                    self.builder.limit = Some(limit);
                    self
                }

                /// Create a flattened paginator
                    ///
                    /// This paginator automatically flattens results using `results`. Queries to the underlying service
                    /// are dispatched lazily.
                    pub fn items(self) -> crate::paginator::DescribeMlModelsPaginatorItems {
                        crate::paginator::DescribeMlModelsPaginatorItems(self)
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
                pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::output::DescribeMlModelsOutput, aws_smithy_http::result::SdkError<crate::error::DescribeMLModelsError>>> + Unpin
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
                                    let new_token = crate::lens::reflens_describe_ml_models_output_next_token(resp);
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

/// Flattened paginator for `DescribeBatchPredictionsPaginator`
                ///
                /// This is created with [`.items()`](DescribeBatchPredictionsPaginator::items)
                pub struct DescribeBatchPredictionsPaginatorItems(DescribeBatchPredictionsPaginator);

                impl  DescribeBatchPredictionsPaginatorItems  {
                    /// Create the pagination stream
                    ///
                    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
                    ///
                    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
                    pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::model::BatchPrediction, aws_smithy_http::result::SdkError<crate::error::DescribeBatchPredictionsError>>> + Unpin
                     {
                        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_describe_batch_predictions_output_results(page).unwrap_or_default().into_iter())
                    }
                }

/// Flattened paginator for `DescribeDataSourcesPaginator`
                ///
                /// This is created with [`.items()`](DescribeDataSourcesPaginator::items)
                pub struct DescribeDataSourcesPaginatorItems(DescribeDataSourcesPaginator);

                impl  DescribeDataSourcesPaginatorItems  {
                    /// Create the pagination stream
                    ///
                    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
                    ///
                    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
                    pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::model::DataSource, aws_smithy_http::result::SdkError<crate::error::DescribeDataSourcesError>>> + Unpin
                     {
                        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_describe_data_sources_output_results(page).unwrap_or_default().into_iter())
                    }
                }

/// Flattened paginator for `DescribeEvaluationsPaginator`
                ///
                /// This is created with [`.items()`](DescribeEvaluationsPaginator::items)
                pub struct DescribeEvaluationsPaginatorItems(DescribeEvaluationsPaginator);

                impl  DescribeEvaluationsPaginatorItems  {
                    /// Create the pagination stream
                    ///
                    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
                    ///
                    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
                    pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::model::Evaluation, aws_smithy_http::result::SdkError<crate::error::DescribeEvaluationsError>>> + Unpin
                     {
                        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_describe_evaluations_output_results(page).unwrap_or_default().into_iter())
                    }
                }

/// Flattened paginator for `DescribeMlModelsPaginator`
                ///
                /// This is created with [`.items()`](DescribeMlModelsPaginator::items)
                pub struct DescribeMlModelsPaginatorItems(DescribeMlModelsPaginator);

                impl  DescribeMlModelsPaginatorItems  {
                    /// Create the pagination stream
                    ///
                    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
                    ///
                    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
                    pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::model::MlModel, aws_smithy_http::result::SdkError<crate::error::DescribeMLModelsError>>> + Unpin
                     {
                        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_describe_ml_models_output_results(page).unwrap_or_default().into_iter())
                    }
                }

