// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeObjects`](crate::client::fluent_builders::DescribeObjects) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeObjects::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`pipeline_id(impl Into<String>)`](crate::client::fluent_builders::DescribeObjects::pipeline_id) / [`set_pipeline_id(Option<String>)`](crate::client::fluent_builders::DescribeObjects::set_pipeline_id): <p>The ID of the pipeline that contains the object definitions.</p>
    ///   - [`object_ids(Vec<String>)`](crate::client::fluent_builders::DescribeObjects::object_ids) / [`set_object_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeObjects::set_object_ids): <p>The IDs of the pipeline objects that contain the definitions to be described. You can pass as many as 25 identifiers in a single call to <code>DescribeObjects</code>.</p>
    ///   - [`evaluate_expressions(bool)`](crate::client::fluent_builders::DescribeObjects::evaluate_expressions) / [`set_evaluate_expressions(bool)`](crate::client::fluent_builders::DescribeObjects::set_evaluate_expressions): <p>Indicates whether any expressions in the object should be evaluated when the object descriptions are returned.</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::DescribeObjects::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::DescribeObjects::set_marker): <p>The starting point for the results to be returned. For the first call, this value should be empty. As long as there are more results, continue to call <code>DescribeObjects</code> with the marker value from the previous call to retrieve the next set of results.</p>
                            /// - On success, responds with [`DescribeObjectsOutput`](crate::output::DescribeObjectsOutput) with field(s):
    ///   - [`pipeline_objects(Option<Vec<PipelineObject>>)`](crate::output::DescribeObjectsOutput::pipeline_objects): <p>An array of object definitions.</p>
    ///   - [`marker(Option<String>)`](crate::output::DescribeObjectsOutput::marker): <p>The starting point for the next page of results. To view the next page of results, call <code>DescribeObjects</code> again with this marker value. If the value is null, there are no more results.</p>
    ///   - [`has_more_results(bool)`](crate::output::DescribeObjectsOutput::has_more_results): <p>Indicates whether there are more results to return.</p>
                            /// - On failure, responds with [`SdkError<DescribeObjectsError>`](crate::error::DescribeObjectsError)
    pub fn describe_objects(&self) -> crate::client::fluent_builders::DescribeObjects {
                                crate::client::fluent_builders::DescribeObjects::new(self.handle.clone())
                            }
}

