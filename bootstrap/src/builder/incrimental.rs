//! Rage Bootstrap
//! Incrimental Build Phases

/// Steps of an incrimental build.
/// Under certain conditions, steps can be skipped.
#[derive(PartialEq)]
pub enum Incrimental {
    /// Check if a build has been cached.
    CheckCached,
    /// Has loaded metadata.
    Metadata {
        // TODO: inner: Metadata   
    },
    /// Source code has been loaded and needs parsed.
    RawText(String)
}
