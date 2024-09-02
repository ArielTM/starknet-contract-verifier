use anyhow::Result;
use camino::Utf8PathBuf;

#[derive(Debug, Clone, Copy)]
pub enum SupportedCairoVersions {
    V2_5_0,
}

impl ToString for SupportedCairoVersions {
    fn to_string(&self) -> String {
        match self {
            SupportedCairoVersions::V2_5_0 => "2.5.0".into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SupportedScarbVersions {
    V2_5_0,
}

impl ToString for SupportedScarbVersions {
    fn to_string(&self) -> String {
        match self {
            // SupportedScarbVersions::V0_4_0 => "0.4.0".into(),
            // SupportedScarbVersions::V0_4_1 => "0.4.1".into(),
            // SupportedScarbVersions::V0_5_0 => "0.5.0".into(),
            // SupportedScarbVersions::V0_5_1 => "0.5.1".into(),
            // SupportedScarbVersions::V0_5_2 => "0.5.2".into(),
            // SupportedScarbVersions::V0_6_1 => "0.6.1".into(),
            // SupportedScarbVersions::V0_6_2 => "0.6.2".into(),
            // SupportedScarbVersions::V0_7_0 => "0.7.0".into(),
            SupportedScarbVersions::V2_5_0 => "2.5.0".into(),
        }
    }
}

/**
 * This trait is required to be implemented by the voyager resolvers.
 * This allows us to use multiple version of scarb + cairo in the same project,
 * and compile scarb projects easily,
 */
pub trait DynamicCompiler {
    fn get_supported_scarb_versions(&self) -> Vec<SupportedScarbVersions>;

    fn get_supported_cairo_versions(&self) -> Vec<SupportedCairoVersions>;

    fn get_contracts_to_verify_path(&self, project_path: &Utf8PathBuf) -> Result<Vec<Utf8PathBuf>>;

    fn compile_project(&self, project_path: &Utf8PathBuf) -> Result<()>;

    fn compile_file(&self, file_path: &Utf8PathBuf) -> Result<()>;
}
