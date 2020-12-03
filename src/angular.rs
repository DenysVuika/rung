//! # Angular utils
//!
//! Provides a collection of utilities to work with Angular configuration.

use anyhow::Result;
use log::info;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::utils;

/// Workspace configuration file
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceConfig {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    /// File format version
    pub version: u32,
    /// Path where new projects will be created.
    pub new_project_root: Option<String>,
    /// Default project name used in commands.
    pub default_project: Option<String>,
    pub projects: Option<HashMap<String, Project>>,
    pub cli: Option<CliOptions>,
    // todo: schematics
}

/// Workspace project
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    /// Project type.
    pub project_type: ProjectType,
    pub cli: Option<CliOptions>,
    /// The prefix to apply to generated selectors.
    pub prefix: Option<String>,
    /// Root of the project files.
    pub root: Option<String>,
    /// The root of the source files, assets and index.html file structure.
    pub source_root: Option<String>,
    // todo: schematics
    // todo: i18n
    // todo: architect
    // todo: targets
}

/// Types of the workspace projects
#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Application,
    Library,
}

/// Angular CLI Options
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CliOptions {
    /// The default schematics collection to use.
    pub default_collection: Option<String>,
    /// Specify which package manager tool to use.
    pub package_manager: Option<PackageManager>,
    /// Control CLI specific console warnings
    pub warnings: Option<CliWarnings>,
    // todo: analytics
}

/// Angular CLI Warnings
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CliWarnings {
    /// Show a warning when the global version is newer than the local one.
    pub version_mismatch: bool,
}

/// Types of the supported package managers
#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PackageManager {
    Npm,
    Cnpm,
    Yarn,
    Pnpm,
}

/// Loads Angular workspace configuration from the file.
pub fn read_config(path: PathBuf) -> Result<WorkspaceConfig> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config: WorkspaceConfig = serde_json::from_reader(reader)?;

    Ok(config)
}

/// Lists the projects within the workspace configuration
pub fn list_projects(config: &WorkspaceConfig) -> Result<()> {
    let projects = &config.projects.as_ref().unwrap();

    for (key, value) in projects.iter() {
        println!("{} ({:?})", key, value.project_type);
    }

    Ok(())
}

/// List workspace projects based on a specific type
pub fn list_projects_by_type(config: &WorkspaceConfig, project_type: ProjectType) -> Result<()> {
    let projects = &config.projects.as_ref().unwrap();

    for (key, value) in projects.iter() {
        if value.project_type == project_type {
            println!("{}", key);
        }
    }

    Ok(())
}

/// Create new Angular application
pub fn new_application(name: &str, dir: &PathBuf) -> Result<bool> {
    info!("Creating new workspace: {}", name);

    std::fs::create_dir_all(dir)?;

    let args = &["new", name, "--skip-install", "--skip-git"];
    let result = utils::exec_command(dir, "ng", args);

    Ok(result)
}
