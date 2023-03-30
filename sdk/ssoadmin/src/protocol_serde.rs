// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_attach_customer_managed_policy_reference_to_permission_set;

pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
                    crate::json_errors::parse_error_metadata(response.body(), response.headers())
                }

pub(crate) mod shape_attach_managed_policy_to_permission_set;

pub(crate) mod shape_create_account_assignment;

pub(crate) mod shape_create_instance_access_control_attribute_configuration;

pub(crate) mod shape_create_permission_set;

pub(crate) mod shape_delete_account_assignment;

pub(crate) mod shape_delete_inline_policy_from_permission_set;

pub(crate) mod shape_delete_instance_access_control_attribute_configuration;

pub(crate) mod shape_delete_permission_set;

pub(crate) mod shape_delete_permissions_boundary_from_permission_set;

pub(crate) mod shape_describe_account_assignment_creation_status;

pub(crate) mod shape_describe_account_assignment_deletion_status;

pub(crate) mod shape_describe_instance_access_control_attribute_configuration;

pub(crate) mod shape_describe_permission_set;

pub(crate) mod shape_describe_permission_set_provisioning_status;

pub(crate) mod shape_detach_customer_managed_policy_reference_from_permission_set;

pub(crate) mod shape_detach_managed_policy_from_permission_set;

pub(crate) mod shape_get_inline_policy_for_permission_set;

pub(crate) mod shape_get_permissions_boundary_for_permission_set;

pub(crate) mod shape_list_account_assignment_creation_status;

pub(crate) mod shape_list_account_assignment_deletion_status;

pub(crate) mod shape_list_account_assignments;

pub(crate) mod shape_list_accounts_for_provisioned_permission_set;

pub(crate) mod shape_list_customer_managed_policy_references_in_permission_set;

pub(crate) mod shape_list_instances;

pub(crate) mod shape_list_managed_policies_in_permission_set;

pub(crate) mod shape_list_permission_set_provisioning_status;

pub(crate) mod shape_list_permission_sets;

pub(crate) mod shape_list_permission_sets_provisioned_to_account;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_provision_permission_set;

pub(crate) mod shape_put_inline_policy_to_permission_set;

pub(crate) mod shape_put_permissions_boundary_to_permission_set;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_instance_access_control_attribute_configuration;

pub(crate) mod shape_update_permission_set;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_attach_customer_managed_policy_reference_to_permission_set_input;

pub(crate) mod shape_attach_managed_policy_to_permission_set_input;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_account_assignment_input;

pub(crate) mod shape_create_instance_access_control_attribute_configuration_input;

pub(crate) mod shape_create_permission_set_input;

pub(crate) mod shape_delete_account_assignment_input;

pub(crate) mod shape_delete_inline_policy_from_permission_set_input;

pub(crate) mod shape_delete_instance_access_control_attribute_configuration_input;

pub(crate) mod shape_delete_permission_set_input;

pub(crate) mod shape_delete_permissions_boundary_from_permission_set_input;

pub(crate) mod shape_describe_account_assignment_creation_status_input;

pub(crate) mod shape_describe_account_assignment_deletion_status_input;

pub(crate) mod shape_describe_instance_access_control_attribute_configuration_input;

pub(crate) mod shape_describe_permission_set_input;

pub(crate) mod shape_describe_permission_set_provisioning_status_input;

pub(crate) mod shape_detach_customer_managed_policy_reference_from_permission_set_input;

pub(crate) mod shape_detach_managed_policy_from_permission_set_input;

pub(crate) mod shape_get_inline_policy_for_permission_set_input;

pub(crate) mod shape_get_permissions_boundary_for_permission_set_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_list_account_assignment_creation_status_input;

pub(crate) mod shape_list_account_assignment_deletion_status_input;

pub(crate) mod shape_list_account_assignments_input;

pub(crate) mod shape_list_accounts_for_provisioned_permission_set_input;

pub(crate) mod shape_list_customer_managed_policy_references_in_permission_set_input;

pub(crate) mod shape_list_instances_input;

pub(crate) mod shape_list_managed_policies_in_permission_set_input;

pub(crate) mod shape_list_permission_set_provisioning_status_input;

pub(crate) mod shape_list_permission_sets_input;

pub(crate) mod shape_list_permission_sets_provisioned_to_account_input;

pub(crate) mod shape_list_tags_for_resource_input;

pub(crate) mod shape_provision_permission_set_input;

pub(crate) mod shape_put_inline_policy_to_permission_set_input;

pub(crate) mod shape_put_permissions_boundary_to_permission_set_input;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_untag_resource_input;

pub(crate) mod shape_update_instance_access_control_attribute_configuration_input;

pub(crate) mod shape_update_permission_set_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_account_assignment_list;

pub(crate) mod shape_account_assignment_operation_status;

pub(crate) mod shape_account_assignment_operation_status_list;

pub(crate) mod shape_account_list;

pub(crate) mod shape_attached_managed_policy_list;

pub(crate) mod shape_customer_managed_policy_reference;

pub(crate) mod shape_customer_managed_policy_reference_list;

pub(crate) mod shape_instance_access_control_attribute_configuration;

pub(crate) mod shape_instance_list;

pub(crate) mod shape_operation_status_filter;

pub(crate) mod shape_permission_set;

pub(crate) mod shape_permission_set_list;

pub(crate) mod shape_permission_set_provisioning_status;

pub(crate) mod shape_permission_set_provisioning_status_list;

pub(crate) mod shape_permissions_boundary;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_access_control_attribute;

pub(crate) mod shape_access_control_attribute_list;

pub(crate) mod shape_account_assignment;

pub(crate) mod shape_account_assignment_operation_status_metadata;

pub(crate) mod shape_attached_managed_policy;

pub(crate) mod shape_instance_metadata;

pub(crate) mod shape_permission_set_provisioning_status_metadata;

pub(crate) mod shape_access_control_attribute_value;

pub(crate) mod shape_access_control_attribute_value_source_list;

