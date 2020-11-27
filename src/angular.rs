use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceConfig {
    #[serde(rename = "$schema")]
    schema: Option<String>,
    /// File format version
    version: u32,
    /// Path where new projects will be created.
    new_project_root: Option<String>,
    /// Default project name used in commands.
    default_project: Option<String>,
    projects: Option<HashMap<String, Project>>,
    cli: Option<CliOptions>,
    // todo: schematics
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    /// Project type.
    project_type: ProjectType,
    cli: Option<CliOptions>,
    /// The prefix to apply to generated selectors.
    prefix: Option<String>,
    /// Root of the project files.
    root: Option<String>,
    /// The root of the source files, assets and index.html file structure.
    source_root: Option<String>,
    // todo: schematics
    // todo: i18n
    // todo: architect
    // todo: targets
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Application,
    Library,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CliOptions {
    /// The default schematics collection to use.
    default_collection: Option<String>,
    /// Specify which package manager tool to use.
    package_manager: Option<PackageManager>,
    /// Control CLI specific console warnings
    warnings: Option<CliWarnings>,
    // todo: analytics
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CliWarnings {
    /// Show a warning when the global version is newer than the local one.
    version_mismatch: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PackageManager {
    Npm,
    Cnpm,
    Yarn,
    Pnpm,
}

pub fn read_config(path: PathBuf) -> Result<WorkspaceConfig> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config: WorkspaceConfig = serde_json::from_reader(reader)?;

    Ok(config)
}

pub fn list_projects(config: &WorkspaceConfig) -> Result<()> {
    let projects = &config.projects.as_ref().unwrap();

    for (key, value) in projects.into_iter() {
        println!("{} ({:?})", key, value.project_type);
    }

    Ok(())
}

pub fn get_config_path() -> Result<PathBuf> {
    let mut path = std::env::current_dir().unwrap();
    path.push("angular.json");

    Ok(path)
}

pub fn list_projects_by_type(config: &WorkspaceConfig, project_type: ProjectType) -> Result<()> {
    let projects = &config.projects.as_ref().unwrap();

    for (key, value) in projects.into_iter() {
        if value.project_type == project_type {
            println!("{}", key);
        }
    }

    Ok(())
}
