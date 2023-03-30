// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListOrganizationPortfolioAccess`](crate::client::fluent_builders::ListOrganizationPortfolioAccess) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListOrganizationPortfolioAccess::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`accept_language(impl Into<String>)`](crate::client::fluent_builders::ListOrganizationPortfolioAccess::accept_language) / [`set_accept_language(Option<String>)`](crate::client::fluent_builders::ListOrganizationPortfolioAccess::set_accept_language): <p>The language code.</p>  <ul>   <li> <p> <code>en</code> - English (default)</p> </li>   <li> <p> <code>jp</code> - Japanese</p> </li>   <li> <p> <code>zh</code> - Chinese</p> </li>  </ul>
    ///   - [`portfolio_id(impl Into<String>)`](crate::client::fluent_builders::ListOrganizationPortfolioAccess::portfolio_id) / [`set_portfolio_id(Option<String>)`](crate::client::fluent_builders::ListOrganizationPortfolioAccess::set_portfolio_id): <p>The portfolio identifier. For example, <code>port-2abcdext3y5fk</code>.</p>
    ///   - [`organization_node_type(OrganizationNodeType)`](crate::client::fluent_builders::ListOrganizationPortfolioAccess::organization_node_type) / [`set_organization_node_type(Option<OrganizationNodeType>)`](crate::client::fluent_builders::ListOrganizationPortfolioAccess::set_organization_node_type): <p>The organization node type that will be returned in the output.</p>  <ul>   <li> <p> <code>ORGANIZATION</code> - Organization that has access to the portfolio. </p> </li>   <li> <p> <code>ORGANIZATIONAL_UNIT</code> - Organizational unit that has access to the portfolio within your organization.</p> </li>   <li> <p> <code>ACCOUNT</code> - Account that has access to the portfolio within your organization.</p> </li>  </ul>
    ///   - [`page_token(impl Into<String>)`](crate::client::fluent_builders::ListOrganizationPortfolioAccess::page_token) / [`set_page_token(Option<String>)`](crate::client::fluent_builders::ListOrganizationPortfolioAccess::set_page_token): <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    ///   - [`page_size(i32)`](crate::client::fluent_builders::ListOrganizationPortfolioAccess::page_size) / [`set_page_size(i32)`](crate::client::fluent_builders::ListOrganizationPortfolioAccess::set_page_size): <p>The maximum number of items to return with this call.</p>
                            /// - On success, responds with [`ListOrganizationPortfolioAccessOutput`](crate::output::ListOrganizationPortfolioAccessOutput) with field(s):
    ///   - [`organization_nodes(Option<Vec<OrganizationNode>>)`](crate::output::ListOrganizationPortfolioAccessOutput::organization_nodes): <p>Displays information about the organization nodes.</p>
    ///   - [`next_page_token(Option<String>)`](crate::output::ListOrganizationPortfolioAccessOutput::next_page_token): <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
                            /// - On failure, responds with [`SdkError<ListOrganizationPortfolioAccessError>`](crate::error::ListOrganizationPortfolioAccessError)
    pub fn list_organization_portfolio_access(&self) -> crate::client::fluent_builders::ListOrganizationPortfolioAccess {
                                crate::client::fluent_builders::ListOrganizationPortfolioAccess::new(self.handle.clone())
                            }
}

