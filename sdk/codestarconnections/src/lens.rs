// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_connections_output_next_token(input: &crate::output::ListConnectionsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_hosts_output_next_token(input: &crate::output::ListHostsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

