// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_certificate_authorities_output_next_token(input: &crate::output::ListCertificateAuthoritiesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_permissions_output_next_token(input: &crate::output::ListPermissionsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_tags_output_next_token(input: &crate::output::ListTagsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_certificate_authorities_output_certificate_authorities(input: crate::output::ListCertificateAuthoritiesOutput) -> std::option::Option<std::vec::Vec<crate::model::CertificateAuthority>> {
                    let input = match input.certificate_authorities {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_permissions_output_permissions(input: crate::output::ListPermissionsOutput) -> std::option::Option<std::vec::Vec<crate::model::Permission>> {
                    let input = match input.permissions {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_tags_output_tags(input: crate::output::ListTagsOutput) -> std::option::Option<std::vec::Vec<crate::model::Tag>> {
                    let input = match input.tags {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

