// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_custom_verification_email_templates_output_next_token(input: &crate::output::ListCustomVerificationEmailTemplatesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_identities_output_next_token(input: &crate::output::ListIdentitiesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_identities_output_identities(input: crate::output::ListIdentitiesOutput) -> std::option::Option<std::vec::Vec<std::string::String>> {
                    let input = match input.identities {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

