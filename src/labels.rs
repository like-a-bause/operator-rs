use const_format::concatcp;
use kube::api::{Resource, ResourceExt};
use std::collections::BTreeMap;

const APP_KUBERNETES_LABEL_BASE: &str = "app.kubernetes.io/";

/// The name of the application e.g. "mysql"
pub const APP_NAME_LABEL: &str = concatcp!(APP_KUBERNETES_LABEL_BASE, "name");
/// A unique name identifying the instance of an application e.g. "mysql-abcxzy"
pub const APP_INSTANCE_LABEL: &str = concatcp!(APP_KUBERNETES_LABEL_BASE, "instance");
/// The current version of the application (e.g., a semantic version, revision hash, etc.) e.g."5.7.21"
pub const APP_VERSION_LABEL: &str = concatcp!(APP_KUBERNETES_LABEL_BASE, "version");
/// The component within the architecture e.g. database
pub const APP_COMPONENT_LABEL: &str = concatcp!(APP_KUBERNETES_LABEL_BASE, "component");
/// The name of a higher level application this one is part of e.g. "wordpress"
pub const APP_PART_OF_LABEL: &str = concatcp!(APP_KUBERNETES_LABEL_BASE, "part-of");
/// The tool being used to manage the operation of an application e.g. helm
pub const APP_MANAGED_BY_LABEL: &str = concatcp!(APP_KUBERNETES_LABEL_BASE, "managed-by");
pub const APP_ROLE_GROUP_LABEL: &str = concatcp!(APP_KUBERNETES_LABEL_BASE, "role-group");

/// Create kubernetes recommended labels
pub fn get_recommended_labels<T>(
    resource: &T,
    app_name: &str,
    app_version: &str,
    app_role: &str,
    app_role_group: &str,
) -> BTreeMap<String, String>
where
    T: Resource,
{
    let mut recommended_labels = BTreeMap::new();

    // TODO: Add operator version label
    // TODO: part-of is empty for now, decide on how this can be used in a proper fashion
    recommended_labels.insert(APP_INSTANCE_LABEL.to_string(), resource.name());
    recommended_labels.insert(APP_NAME_LABEL.to_string(), app_name.to_string());
    recommended_labels.insert(APP_VERSION_LABEL.to_string(), app_version.to_string());
    recommended_labels.insert(APP_COMPONENT_LABEL.to_string(), app_role.to_string());
    recommended_labels.insert(APP_ROLE_GROUP_LABEL.to_string(), app_role_group.to_string());
    recommended_labels.insert(
        APP_MANAGED_BY_LABEL.to_string(),
        format!("{}-operator", app_name),
    );

    recommended_labels
}

/// The APP_NAME_LABEL (Spark, Kafka, ZooKeeper...) and APP_INSTANCES_LABEL (simple, test ...) are
/// required to identify resources that belong to a certain Custom Resource.
pub fn build_common_labels_for_all_managed_resources(
    app_name: &str,
    app_instance: &str,
) -> BTreeMap<String, String> {
    let mut labels = BTreeMap::new();
    labels.insert(APP_NAME_LABEL.to_string(), app_name.to_string());
    labels.insert(APP_INSTANCE_LABEL.to_string(), app_instance.to_string());
    labels
}
