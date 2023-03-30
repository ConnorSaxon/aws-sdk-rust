// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeAlgorithm`](crate::client::fluent_builders::DescribeAlgorithm) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`algorithm_name(impl Into<String>)`](crate::client::fluent_builders::DescribeAlgorithm::algorithm_name) / [`set_algorithm_name(Option<String>)`](crate::client::fluent_builders::DescribeAlgorithm::set_algorithm_name): <p>The name of the algorithm to describe.</p>
                            /// - On success, responds with [`DescribeAlgorithmOutput`](crate::output::DescribeAlgorithmOutput) with field(s):
    ///   - [`algorithm_name(Option<String>)`](crate::output::DescribeAlgorithmOutput::algorithm_name): <p>The name of the algorithm being described.</p>
    ///   - [`algorithm_arn(Option<String>)`](crate::output::DescribeAlgorithmOutput::algorithm_arn): <p>The Amazon Resource Name (ARN) of the algorithm.</p>
    ///   - [`algorithm_description(Option<String>)`](crate::output::DescribeAlgorithmOutput::algorithm_description): <p>A brief summary about the algorithm.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::output::DescribeAlgorithmOutput::creation_time): <p>A timestamp specifying when the algorithm was created.</p>
    ///   - [`training_specification(Option<TrainingSpecification>)`](crate::output::DescribeAlgorithmOutput::training_specification): <p>Details about training jobs run by this algorithm.</p>
    ///   - [`inference_specification(Option<InferenceSpecification>)`](crate::output::DescribeAlgorithmOutput::inference_specification): <p>Details about inference jobs that the algorithm runs.</p>
    ///   - [`validation_specification(Option<AlgorithmValidationSpecification>)`](crate::output::DescribeAlgorithmOutput::validation_specification): <p>Details about configurations for one or more training jobs that SageMaker runs to test the algorithm.</p>
    ///   - [`algorithm_status(Option<AlgorithmStatus>)`](crate::output::DescribeAlgorithmOutput::algorithm_status): <p>The current status of the algorithm.</p>
    ///   - [`algorithm_status_details(Option<AlgorithmStatusDetails>)`](crate::output::DescribeAlgorithmOutput::algorithm_status_details): <p>Details about the current status of the algorithm.</p>
    ///   - [`product_id(Option<String>)`](crate::output::DescribeAlgorithmOutput::product_id): <p>The product identifier of the algorithm.</p>
    ///   - [`certify_for_marketplace(bool)`](crate::output::DescribeAlgorithmOutput::certify_for_marketplace): <p>Whether the algorithm is certified to be listed in Amazon Web Services Marketplace.</p>
                            /// - On failure, responds with [`SdkError<DescribeAlgorithmError>`](crate::error::DescribeAlgorithmError)
    pub fn describe_algorithm(&self) -> crate::client::fluent_builders::DescribeAlgorithm {
                                crate::client::fluent_builders::DescribeAlgorithm::new(self.handle.clone())
                            }
}

