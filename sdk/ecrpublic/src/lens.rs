// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_describe_images_output_next_token(input: &crate::output::DescribeImagesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_describe_image_tags_output_next_token(input: &crate::output::DescribeImageTagsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_describe_registries_output_next_token(input: &crate::output::DescribeRegistriesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_describe_repositories_output_next_token(input: &crate::output::DescribeRepositoriesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_describe_images_output_image_details(input: crate::output::DescribeImagesOutput) -> std::option::Option<std::vec::Vec<crate::model::ImageDetail>> {
                    let input = match input.image_details {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_describe_image_tags_output_image_tag_details(input: crate::output::DescribeImageTagsOutput) -> std::option::Option<std::vec::Vec<crate::model::ImageTagDetail>> {
                    let input = match input.image_tag_details {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_describe_registries_output_registries(input: crate::output::DescribeRegistriesOutput) -> std::option::Option<std::vec::Vec<crate::model::Registry>> {
                    let input = match input.registries {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_describe_repositories_output_repositories(input: crate::output::DescribeRepositoriesOutput) -> std::option::Option<std::vec::Vec<crate::model::Repository>> {
                    let input = match input.repositories {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

