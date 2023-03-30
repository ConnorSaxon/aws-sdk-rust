// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_describe_addresses_output_next_token(input: &crate::output::DescribeAddressesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_cluster_jobs_output_next_token(input: &crate::output::ListClusterJobsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_clusters_output_next_token(input: &crate::output::ListClustersOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_compatible_images_output_next_token(input: &crate::output::ListCompatibleImagesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_jobs_output_next_token(input: &crate::output::ListJobsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_long_term_pricing_output_next_token(input: &crate::output::ListLongTermPricingOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_describe_addresses_output_addresses(input: crate::output::DescribeAddressesOutput) -> std::option::Option<std::vec::Vec<crate::model::Address>> {
                    let input = match input.addresses {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_cluster_jobs_output_job_list_entries(input: crate::output::ListClusterJobsOutput) -> std::option::Option<std::vec::Vec<crate::model::JobListEntry>> {
                    let input = match input.job_list_entries {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_clusters_output_cluster_list_entries(input: crate::output::ListClustersOutput) -> std::option::Option<std::vec::Vec<crate::model::ClusterListEntry>> {
                    let input = match input.cluster_list_entries {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_compatible_images_output_compatible_images(input: crate::output::ListCompatibleImagesOutput) -> std::option::Option<std::vec::Vec<crate::model::CompatibleImage>> {
                    let input = match input.compatible_images {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_jobs_output_job_list_entries(input: crate::output::ListJobsOutput) -> std::option::Option<std::vec::Vec<crate::model::JobListEntry>> {
                    let input = match input.job_list_entries {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_long_term_pricing_output_long_term_pricing_entries(input: crate::output::ListLongTermPricingOutput) -> std::option::Option<std::vec::Vec<crate::model::LongTermPricingListEntry>> {
                    let input = match input.long_term_pricing_entries {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

