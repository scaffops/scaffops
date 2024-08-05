//! The application.

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use miette::Diagnostic;
use thiserror::Error;

use crate::init::init;

/// Represents sources of errors that can occur during application runs.
#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum ErrorSource {
    /// Initialization errors.
    Init(#[from] crate::init::Error),
}

/// Represents errors that can occur during application runs.
#[derive(Debug, Error, Diagnostic)]
#[error("error encountered")]
#[diagnostic(code(scaffops::app::run), help("see the report for more information"))]
pub struct Error {
    /// The source of this error.
    #[source]
    #[diagnostic_source]
    pub source: ErrorSource,
}

impl Error {
    /// Constructs [`Self`].
    pub fn new(source: ErrorSource) -> Self {
        Self { source }
    }

    /// Constructs [`Self`] from [`Error`].
    ///
    /// [`Error`]: crate::init::Error
    pub fn init(source: crate::init::Error) -> Self {
        Self::new(source.into())
    }
}

/// Represents global options of `scaffops`.
#[derive(Debug, Args)]
pub struct Globals {
    /// The directory to change to before doing anything.
    #[arg(
        short = 'C',
        long,
        global = true,
        name = "DIRECTORY",
        help = "Change to this directory before doing anything"
    )]
    pub directory: Option<PathBuf>,
}

/// Represents the `scaffops` application.
#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about,
    propagate_version = true,
    arg_required_else_help = true
)]
pub struct App {
    /// The global options to use.
    #[command(flatten)]
    pub globals: Globals,
    /// The subcommand to run, if any.
    #[command(subcommand)]
    pub command: Option<Command>,
}

impl App {
    /// Runs the application.
    ///
    /// # Errors
    ///
    /// Returns [`struct@Error`] when any error is encountered.
    pub fn run(self) -> Result<(), Error> {
        let globals = self.globals;

        init(globals.directory).map_err(|error| Error::init(error))?;

        if let Some(command) = self.command {
            match command {}
        };

        Ok(())
    }
}

/// Represents `scaffops` subcommands.
#[derive(Debug, Subcommand)]
pub enum Command {}
