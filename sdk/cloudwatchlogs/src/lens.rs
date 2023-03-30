// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_describe_destinations_output_next_token(input: &crate::output::DescribeDestinationsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_describe_log_groups_output_next_token(input: &crate::output::DescribeLogGroupsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_describe_log_streams_output_next_token(input: &crate::output::DescribeLogStreamsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_describe_metric_filters_output_next_token(input: &crate::output::DescribeMetricFiltersOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_describe_subscription_filters_output_next_token(input: &crate::output::DescribeSubscriptionFiltersOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_filter_log_events_output_next_token(input: &crate::output::FilterLogEventsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_get_log_events_output_next_forward_token(input: &crate::output::GetLogEventsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_forward_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_describe_destinations_output_destinations(input: crate::output::DescribeDestinationsOutput) -> std::option::Option<std::vec::Vec<crate::model::Destination>> {
                    let input = match input.destinations {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_describe_log_groups_output_log_groups(input: crate::output::DescribeLogGroupsOutput) -> std::option::Option<std::vec::Vec<crate::model::LogGroup>> {
                    let input = match input.log_groups {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_describe_log_streams_output_log_streams(input: crate::output::DescribeLogStreamsOutput) -> std::option::Option<std::vec::Vec<crate::model::LogStream>> {
                    let input = match input.log_streams {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_describe_metric_filters_output_metric_filters(input: crate::output::DescribeMetricFiltersOutput) -> std::option::Option<std::vec::Vec<crate::model::MetricFilter>> {
                    let input = match input.metric_filters {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_describe_subscription_filters_output_subscription_filters(input: crate::output::DescribeSubscriptionFiltersOutput) -> std::option::Option<std::vec::Vec<crate::model::SubscriptionFilter>> {
                    let input = match input.subscription_filters {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_get_log_events_output_events(input: crate::output::GetLogEventsOutput) -> std::option::Option<std::vec::Vec<crate::model::OutputLogEvent>> {
                    let input = match input.events {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

