// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SearchProducts`](crate::client::fluent_builders::SearchProducts) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::SearchProducts::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`accept_language(impl Into<String>)`](crate::client::fluent_builders::SearchProducts::accept_language) / [`set_accept_language(Option<String>)`](crate::client::fluent_builders::SearchProducts::set_accept_language): <p>The language code.</p>  <ul>   <li> <p> <code>en</code> - English (default)</p> </li>   <li> <p> <code>jp</code> - Japanese</p> </li>   <li> <p> <code>zh</code> - Chinese</p> </li>  </ul>
    ///   - [`filters(HashMap<ProductViewFilterBy, Vec<String>>)`](crate::client::fluent_builders::SearchProducts::filters) / [`set_filters(Option<HashMap<ProductViewFilterBy, Vec<String>>>)`](crate::client::fluent_builders::SearchProducts::set_filters): <p>The search filters. If no search filters are specified, the output includes all products to which the caller has access.</p>
    ///   - [`page_size(i32)`](crate::client::fluent_builders::SearchProducts::page_size) / [`set_page_size(i32)`](crate::client::fluent_builders::SearchProducts::set_page_size): <p>The maximum number of items to return with this call.</p>
    ///   - [`sort_by(ProductViewSortBy)`](crate::client::fluent_builders::SearchProducts::sort_by) / [`set_sort_by(Option<ProductViewSortBy>)`](crate::client::fluent_builders::SearchProducts::set_sort_by): <p>The sort field. If no value is specified, the results are not sorted.</p>
    ///   - [`sort_order(SortOrder)`](crate::client::fluent_builders::SearchProducts::sort_order) / [`set_sort_order(Option<SortOrder>)`](crate::client::fluent_builders::SearchProducts::set_sort_order): <p>The sort order. If no value is specified, the results are not sorted.</p>
    ///   - [`page_token(impl Into<String>)`](crate::client::fluent_builders::SearchProducts::page_token) / [`set_page_token(Option<String>)`](crate::client::fluent_builders::SearchProducts::set_page_token): <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
                            /// - On success, responds with [`SearchProductsOutput`](crate::output::SearchProductsOutput) with field(s):
    ///   - [`product_view_summaries(Option<Vec<ProductViewSummary>>)`](crate::output::SearchProductsOutput::product_view_summaries): <p>Information about the product views.</p>
    ///   - [`product_view_aggregations(Option<HashMap<String, Vec<ProductViewAggregationValue>>>)`](crate::output::SearchProductsOutput::product_view_aggregations): <p>The product view aggregations.</p>
    ///   - [`next_page_token(Option<String>)`](crate::output::SearchProductsOutput::next_page_token): <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
                            /// - On failure, responds with [`SdkError<SearchProductsError>`](crate::error::SearchProductsError)
    pub fn search_products(&self) -> crate::client::fluent_builders::SearchProducts {
                                crate::client::fluent_builders::SearchProducts::new(self.handle.clone())
                            }
}

