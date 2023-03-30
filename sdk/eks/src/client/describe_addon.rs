// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeAddon`](crate::client::fluent_builders::DescribeAddon) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cluster_name(impl Into<String>)`](crate::client::fluent_builders::DescribeAddon::cluster_name) / [`set_cluster_name(Option<String>)`](crate::client::fluent_builders::DescribeAddon::set_cluster_name): <p>The name of the cluster.</p>
    ///   - [`addon_name(impl Into<String>)`](crate::client::fluent_builders::DescribeAddon::addon_name) / [`set_addon_name(Option<String>)`](crate::client::fluent_builders::DescribeAddon::set_addon_name): <p>The name of the add-on. The name must match one of the names returned by <a href="https://docs.aws.amazon.com/eks/latest/APIReference/API_ListAddons.html"> <code>ListAddons</code> </a>.</p>
                            /// - On success, responds with [`DescribeAddonOutput`](crate::output::DescribeAddonOutput) with field(s):
    ///   - [`addon(Option<Addon>)`](crate::output::DescribeAddonOutput::addon): <p>An Amazon EKS add-on. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/eks-add-ons.html">Amazon EKS add-ons</a> in the <i>Amazon EKS User Guide</i>.</p>
                            /// - On failure, responds with [`SdkError<DescribeAddonError>`](crate::error::DescribeAddonError)
    pub fn describe_addon(&self) -> crate::client::fluent_builders::DescribeAddon {
                                crate::client::fluent_builders::DescribeAddon::new(self.handle.clone())
                            }
}

