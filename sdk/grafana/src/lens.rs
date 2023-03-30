// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_permissions_output_next_token(input: &crate::output::ListPermissionsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_workspaces_output_next_token(input: &crate::output::ListWorkspacesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_permissions_output_permissions(input: crate::output::ListPermissionsOutput) -> std::option::Option<std::vec::Vec<crate::model::PermissionEntry>> {
                    let input = match input.permissions {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_workspaces_output_workspaces(input: crate::output::ListWorkspacesOutput) -> std::option::Option<std::vec::Vec<crate::model::WorkspaceSummary>> {
                    let input = match input.workspaces {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

