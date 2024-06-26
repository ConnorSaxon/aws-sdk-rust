// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn version_correct_errors(mut builder: crate::types::builders::VersionBuilder) -> crate::types::builders::VersionBuilder {
    if builder.application_id.is_none() {
        builder.application_id = Some(Default::default())
    }
    if builder.creation_time.is_none() {
        builder.creation_time = Some(Default::default())
    }
    if builder.parameter_definitions.is_none() {
        builder.parameter_definitions = Some(Default::default())
    }
    if builder.required_capabilities.is_none() {
        builder.required_capabilities = Some(Default::default())
    }
    if builder.resources_supported.is_none() {
        builder.resources_supported = Some(Default::default())
    }
    if builder.semantic_version.is_none() {
        builder.semantic_version = Some(Default::default())
    }
    if builder.template_url.is_none() {
        builder.template_url = Some(Default::default())
    }
    builder
}

pub(crate) fn application_dependency_summary_correct_errors(
    mut builder: crate::types::builders::ApplicationDependencySummaryBuilder,
) -> crate::types::builders::ApplicationDependencySummaryBuilder {
    if builder.application_id.is_none() {
        builder.application_id = Some(Default::default())
    }
    if builder.semantic_version.is_none() {
        builder.semantic_version = Some(Default::default())
    }
    builder
}

pub(crate) fn application_policy_statement_correct_errors(
    mut builder: crate::types::builders::ApplicationPolicyStatementBuilder,
) -> crate::types::builders::ApplicationPolicyStatementBuilder {
    if builder.actions.is_none() {
        builder.actions = Some(Default::default())
    }
    if builder.principals.is_none() {
        builder.principals = Some(Default::default())
    }
    builder
}

pub(crate) fn application_summary_correct_errors(
    mut builder: crate::types::builders::ApplicationSummaryBuilder,
) -> crate::types::builders::ApplicationSummaryBuilder {
    if builder.application_id.is_none() {
        builder.application_id = Some(Default::default())
    }
    if builder.author.is_none() {
        builder.author = Some(Default::default())
    }
    if builder.description.is_none() {
        builder.description = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    builder
}

pub(crate) fn parameter_definition_correct_errors(
    mut builder: crate::types::builders::ParameterDefinitionBuilder,
) -> crate::types::builders::ParameterDefinitionBuilder {
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.referenced_by_resources.is_none() {
        builder.referenced_by_resources = Some(Default::default())
    }
    builder
}

pub(crate) fn version_summary_correct_errors(
    mut builder: crate::types::builders::VersionSummaryBuilder,
) -> crate::types::builders::VersionSummaryBuilder {
    if builder.application_id.is_none() {
        builder.application_id = Some(Default::default())
    }
    if builder.creation_time.is_none() {
        builder.creation_time = Some(Default::default())
    }
    if builder.semantic_version.is_none() {
        builder.semantic_version = Some(Default::default())
    }
    builder
}
