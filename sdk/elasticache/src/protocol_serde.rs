// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    crate::rest_xml_wrapped_errors::parse_error_metadata(response.body().as_ref())
}

pub(crate) mod shape_add_tags_to_resource;

pub(crate) mod shape_add_tags_to_resource_input;

pub(crate) mod shape_authorize_cache_security_group_ingress;

pub(crate) mod shape_authorize_cache_security_group_ingress_input;

pub(crate) mod shape_batch_apply_update_action;

pub(crate) mod shape_batch_apply_update_action_input;

pub(crate) mod shape_batch_stop_update_action;

pub(crate) mod shape_batch_stop_update_action_input;

pub(crate) mod shape_complete_migration;

pub(crate) mod shape_complete_migration_input;

pub(crate) mod shape_copy_snapshot;

pub(crate) mod shape_copy_snapshot_input;

pub(crate) mod shape_create_cache_cluster;

pub(crate) mod shape_create_cache_cluster_input;

pub(crate) mod shape_create_cache_parameter_group;

pub(crate) mod shape_create_cache_parameter_group_input;

pub(crate) mod shape_create_cache_security_group;

pub(crate) mod shape_create_cache_security_group_input;

pub(crate) mod shape_create_cache_subnet_group;

pub(crate) mod shape_create_cache_subnet_group_input;

pub(crate) mod shape_create_global_replication_group;

pub(crate) mod shape_create_global_replication_group_input;

pub(crate) mod shape_create_replication_group;

pub(crate) mod shape_create_replication_group_input;

pub(crate) mod shape_create_snapshot;

pub(crate) mod shape_create_snapshot_input;

pub(crate) mod shape_create_user;

pub(crate) mod shape_create_user_group;

pub(crate) mod shape_create_user_group_input;

pub(crate) mod shape_create_user_input;

pub(crate) mod shape_decrease_node_groups_in_global_replication_group;

pub(crate) mod shape_decrease_node_groups_in_global_replication_group_input;

pub(crate) mod shape_decrease_replica_count;

pub(crate) mod shape_decrease_replica_count_input;

pub(crate) mod shape_delete_cache_cluster;

pub(crate) mod shape_delete_cache_cluster_input;

pub(crate) mod shape_delete_cache_parameter_group;

pub(crate) mod shape_delete_cache_parameter_group_input;

pub(crate) mod shape_delete_cache_security_group;

pub(crate) mod shape_delete_cache_security_group_input;

pub(crate) mod shape_delete_cache_subnet_group;

pub(crate) mod shape_delete_cache_subnet_group_input;

pub(crate) mod shape_delete_global_replication_group;

pub(crate) mod shape_delete_global_replication_group_input;

pub(crate) mod shape_delete_replication_group;

pub(crate) mod shape_delete_replication_group_input;

pub(crate) mod shape_delete_snapshot;

pub(crate) mod shape_delete_snapshot_input;

pub(crate) mod shape_delete_user;

pub(crate) mod shape_delete_user_group;

pub(crate) mod shape_delete_user_group_input;

pub(crate) mod shape_delete_user_input;

pub(crate) mod shape_describe_cache_clusters;

pub(crate) mod shape_describe_cache_clusters_input;

pub(crate) mod shape_describe_cache_engine_versions;

pub(crate) mod shape_describe_cache_engine_versions_input;

pub(crate) mod shape_describe_cache_parameter_groups;

pub(crate) mod shape_describe_cache_parameter_groups_input;

pub(crate) mod shape_describe_cache_parameters;

pub(crate) mod shape_describe_cache_parameters_input;

pub(crate) mod shape_describe_cache_security_groups;

pub(crate) mod shape_describe_cache_security_groups_input;

pub(crate) mod shape_describe_cache_subnet_groups;

pub(crate) mod shape_describe_cache_subnet_groups_input;

pub(crate) mod shape_describe_engine_default_parameters;

pub(crate) mod shape_describe_engine_default_parameters_input;

pub(crate) mod shape_describe_events;

pub(crate) mod shape_describe_events_input;

pub(crate) mod shape_describe_global_replication_groups;

pub(crate) mod shape_describe_global_replication_groups_input;

pub(crate) mod shape_describe_replication_groups;

pub(crate) mod shape_describe_replication_groups_input;

pub(crate) mod shape_describe_reserved_cache_nodes;

pub(crate) mod shape_describe_reserved_cache_nodes_input;

pub(crate) mod shape_describe_reserved_cache_nodes_offerings;

pub(crate) mod shape_describe_reserved_cache_nodes_offerings_input;

pub(crate) mod shape_describe_service_updates;

pub(crate) mod shape_describe_service_updates_input;

pub(crate) mod shape_describe_snapshots;

pub(crate) mod shape_describe_snapshots_input;

pub(crate) mod shape_describe_update_actions;

pub(crate) mod shape_describe_update_actions_input;

pub(crate) mod shape_describe_user_groups;

pub(crate) mod shape_describe_user_groups_input;

pub(crate) mod shape_describe_users;

pub(crate) mod shape_describe_users_input;

pub(crate) mod shape_disassociate_global_replication_group;

pub(crate) mod shape_disassociate_global_replication_group_input;

pub(crate) mod shape_failover_global_replication_group;

pub(crate) mod shape_failover_global_replication_group_input;

pub(crate) mod shape_increase_node_groups_in_global_replication_group;

pub(crate) mod shape_increase_node_groups_in_global_replication_group_input;

pub(crate) mod shape_increase_replica_count;

pub(crate) mod shape_increase_replica_count_input;

pub(crate) mod shape_list_allowed_node_type_modifications;

pub(crate) mod shape_list_allowed_node_type_modifications_input;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_list_tags_for_resource_input;

pub(crate) mod shape_modify_cache_cluster;

pub(crate) mod shape_modify_cache_cluster_input;

pub(crate) mod shape_modify_cache_parameter_group;

pub(crate) mod shape_modify_cache_parameter_group_input;

pub(crate) mod shape_modify_cache_subnet_group;

pub(crate) mod shape_modify_cache_subnet_group_input;

pub(crate) mod shape_modify_global_replication_group;

pub(crate) mod shape_modify_global_replication_group_input;

pub(crate) mod shape_modify_replication_group;

pub(crate) mod shape_modify_replication_group_input;

pub(crate) mod shape_modify_replication_group_shard_configuration;

pub(crate) mod shape_modify_replication_group_shard_configuration_input;

pub(crate) mod shape_modify_user;

pub(crate) mod shape_modify_user_group;

pub(crate) mod shape_modify_user_group_input;

pub(crate) mod shape_modify_user_input;

pub(crate) mod shape_purchase_reserved_cache_nodes_offering;

pub(crate) mod shape_purchase_reserved_cache_nodes_offering_input;

pub(crate) mod shape_rebalance_slots_in_global_replication_group;

pub(crate) mod shape_rebalance_slots_in_global_replication_group_input;

pub(crate) mod shape_reboot_cache_cluster;

pub(crate) mod shape_reboot_cache_cluster_input;

pub(crate) mod shape_remove_tags_from_resource;

pub(crate) mod shape_remove_tags_from_resource_input;

pub(crate) mod shape_reset_cache_parameter_group;

pub(crate) mod shape_reset_cache_parameter_group_input;

pub(crate) mod shape_revoke_cache_security_group_ingress;

pub(crate) mod shape_revoke_cache_security_group_ingress_input;

pub(crate) mod shape_start_migration;

pub(crate) mod shape_start_migration_input;

pub(crate) mod shape_test_failover;

pub(crate) mod shape_test_failover_input;

pub(crate) mod shape_api_call_rate_for_customer_exceeded_fault;

pub(crate) mod shape_authentication_mode;

pub(crate) mod shape_authorization_already_exists_fault;

pub(crate) mod shape_authorization_not_found_fault;

pub(crate) mod shape_cache_cluster_already_exists_fault;

pub(crate) mod shape_cache_cluster_not_found_fault;

pub(crate) mod shape_cache_parameter_group_already_exists_fault;

pub(crate) mod shape_cache_parameter_group_not_found_fault;

pub(crate) mod shape_cache_parameter_group_quota_exceeded_fault;

pub(crate) mod shape_cache_security_group_already_exists_fault;

pub(crate) mod shape_cache_security_group_not_found_fault;

pub(crate) mod shape_cache_security_group_quota_exceeded_fault;

pub(crate) mod shape_cache_subnet_group_already_exists_fault;

pub(crate) mod shape_cache_subnet_group_in_use;

pub(crate) mod shape_cache_subnet_group_not_found_fault;

pub(crate) mod shape_cache_subnet_group_quota_exceeded_fault;

pub(crate) mod shape_cache_subnet_quota_exceeded_fault;

pub(crate) mod shape_cluster_quota_for_customer_exceeded_fault;

pub(crate) mod shape_configure_shard;

pub(crate) mod shape_customer_node_endpoint;

pub(crate) mod shape_default_user_associated_to_user_group_fault;

pub(crate) mod shape_default_user_required;

pub(crate) mod shape_duplicate_user_name_fault;

pub(crate) mod shape_filter;

pub(crate) mod shape_global_replication_group_already_exists_fault;

pub(crate) mod shape_global_replication_group_not_found_fault;

pub(crate) mod shape_insufficient_cache_cluster_capacity_fault;

pub(crate) mod shape_invalid_arn_fault;

pub(crate) mod shape_invalid_cache_cluster_state_fault;

pub(crate) mod shape_invalid_cache_parameter_group_state_fault;

pub(crate) mod shape_invalid_cache_security_group_state_fault;

pub(crate) mod shape_invalid_global_replication_group_state_fault;

pub(crate) mod shape_invalid_kms_key_fault;

pub(crate) mod shape_invalid_parameter_combination_exception;

pub(crate) mod shape_invalid_parameter_value_exception;

pub(crate) mod shape_invalid_replication_group_state_fault;

pub(crate) mod shape_invalid_snapshot_state_fault;

pub(crate) mod shape_invalid_subnet;

pub(crate) mod shape_invalid_user_group_state_fault;

pub(crate) mod shape_invalid_user_state_fault;

pub(crate) mod shape_invalid_vpc_network_state_fault;

pub(crate) mod shape_log_delivery_configuration_request;

pub(crate) mod shape_no_operation_fault;

pub(crate) mod shape_node_group_configuration;

pub(crate) mod shape_node_group_not_found_fault;

pub(crate) mod shape_node_groups_per_replication_group_quota_exceeded_fault;

pub(crate) mod shape_node_quota_for_cluster_exceeded_fault;

pub(crate) mod shape_node_quota_for_customer_exceeded_fault;

pub(crate) mod shape_parameter_name_value;

pub(crate) mod shape_regional_configuration;

pub(crate) mod shape_replication_group_already_exists_fault;

pub(crate) mod shape_replication_group_already_under_migration_fault;

pub(crate) mod shape_replication_group_not_found_fault;

pub(crate) mod shape_replication_group_not_under_migration_fault;

pub(crate) mod shape_reserved_cache_node_already_exists_fault;

pub(crate) mod shape_reserved_cache_node_not_found_fault;

pub(crate) mod shape_reserved_cache_node_quota_exceeded_fault;

pub(crate) mod shape_reserved_cache_nodes_offering_not_found_fault;

pub(crate) mod shape_resharding_configuration;

pub(crate) mod shape_service_linked_role_not_found_fault;

pub(crate) mod shape_service_update_not_found_fault;

pub(crate) mod shape_snapshot_already_exists_fault;

pub(crate) mod shape_snapshot_feature_not_supported_fault;

pub(crate) mod shape_snapshot_not_found_fault;

pub(crate) mod shape_snapshot_quota_exceeded_fault;

pub(crate) mod shape_subnet_in_use;

pub(crate) mod shape_subnet_not_allowed_fault;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_not_found_fault;

pub(crate) mod shape_tag_quota_per_resource_exceeded;

pub(crate) mod shape_test_failover_not_available_fault;

pub(crate) mod shape_time_range_filter;

pub(crate) mod shape_user_already_exists_fault;

pub(crate) mod shape_user_group_already_exists_fault;

pub(crate) mod shape_user_group_not_found_fault;

pub(crate) mod shape_user_group_quota_exceeded_fault;

pub(crate) mod shape_user_not_found_fault;

pub(crate) mod shape_user_quota_exceeded_fault;

pub(crate) mod shape_authentication;

pub(crate) mod shape_cache_cluster;

pub(crate) mod shape_cache_cluster_list;

pub(crate) mod shape_cache_engine_version_list;

pub(crate) mod shape_cache_node_type_specific_parameters_list;

pub(crate) mod shape_cache_parameter_group;

pub(crate) mod shape_cache_parameter_group_list;

pub(crate) mod shape_cache_security_group;

pub(crate) mod shape_cache_security_groups;

pub(crate) mod shape_cache_subnet_group;

pub(crate) mod shape_cache_subnet_groups;

pub(crate) mod shape_destination_details;

pub(crate) mod shape_engine_defaults;

pub(crate) mod shape_event_list;

pub(crate) mod shape_global_replication_group;

pub(crate) mod shape_global_replication_group_list;

pub(crate) mod shape_node_type_list;

pub(crate) mod shape_parameters_list;

pub(crate) mod shape_processed_update_action_list;

pub(crate) mod shape_replication_group;

pub(crate) mod shape_replication_group_list;

pub(crate) mod shape_reserved_cache_node;

pub(crate) mod shape_reserved_cache_node_list;

pub(crate) mod shape_reserved_cache_nodes_offering_list;

pub(crate) mod shape_service_update_list;

pub(crate) mod shape_snapshot;

pub(crate) mod shape_snapshot_list;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_ug_replication_group_id_list;

pub(crate) mod shape_unprocessed_update_action_list;

pub(crate) mod shape_update_action_list;

pub(crate) mod shape_user_group_id_list;

pub(crate) mod shape_user_group_list;

pub(crate) mod shape_user_group_pending_changes;

pub(crate) mod shape_user_id_list;

pub(crate) mod shape_user_list;

pub(crate) mod shape_cache_engine_version;

pub(crate) mod shape_cache_node_list;

pub(crate) mod shape_cache_node_type_specific_parameter;

pub(crate) mod shape_cache_parameter_group_status;

pub(crate) mod shape_cache_security_group_membership_list;

pub(crate) mod shape_cloud_watch_logs_destination_details;

pub(crate) mod shape_cluster_id_list;

pub(crate) mod shape_ec2_security_group_list;

pub(crate) mod shape_endpoint;

pub(crate) mod shape_event;

pub(crate) mod shape_global_node_group_list;

pub(crate) mod shape_global_replication_group_info;

pub(crate) mod shape_global_replication_group_member_list;

pub(crate) mod shape_kinesis_firehose_destination_details;

pub(crate) mod shape_log_delivery_configuration_list;

pub(crate) mod shape_network_type_list;

pub(crate) mod shape_node_group_list;

pub(crate) mod shape_node_snapshot_list;

pub(crate) mod shape_notification_configuration;

pub(crate) mod shape_parameter;

pub(crate) mod shape_pending_modified_values;

pub(crate) mod shape_processed_update_action;

pub(crate) mod shape_recurring_charge_list;

pub(crate) mod shape_replication_group_outpost_arn_list;

pub(crate) mod shape_replication_group_pending_modified_values;

pub(crate) mod shape_reserved_cache_nodes_offering;

pub(crate) mod shape_security_group_membership_list;

pub(crate) mod shape_service_update;

pub(crate) mod shape_subnet_list;

pub(crate) mod shape_unprocessed_update_action;

pub(crate) mod shape_update_action;

pub(crate) mod shape_user;

pub(crate) mod shape_user_group;

pub(crate) mod shape_cache_node;

pub(crate) mod shape_cache_node_ids_list;

pub(crate) mod shape_cache_node_type_specific_value_list;

pub(crate) mod shape_cache_node_update_status_list;

pub(crate) mod shape_cache_security_group_membership;

pub(crate) mod shape_ec2_security_group;

pub(crate) mod shape_global_node_group;

pub(crate) mod shape_global_replication_group_member;

pub(crate) mod shape_log_delivery_configuration;

pub(crate) mod shape_node_group;

pub(crate) mod shape_node_group_update_status_list;

pub(crate) mod shape_node_snapshot;

pub(crate) mod shape_pending_log_delivery_configuration_list;

pub(crate) mod shape_recurring_charge;

pub(crate) mod shape_resharding_status;

pub(crate) mod shape_security_group_membership;

pub(crate) mod shape_subnet;

pub(crate) mod shape_user_groups_update_status;

pub(crate) mod shape_availability_zone;

pub(crate) mod shape_cache_node_type_specific_value;

pub(crate) mod shape_cache_node_update_status;

pub(crate) mod shape_node_group_member_list;

pub(crate) mod shape_node_group_update_status;

pub(crate) mod shape_pending_log_delivery_configuration;

pub(crate) mod shape_slot_migration;

pub(crate) mod shape_subnet_outpost;

pub(crate) mod shape_availability_zones_list;

pub(crate) mod shape_node_group_member;

pub(crate) mod shape_node_group_member_update_status_list;

pub(crate) mod shape_outpost_arns_list;

pub(crate) mod shape_node_group_member_update_status;

