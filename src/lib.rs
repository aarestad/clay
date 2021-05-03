//! Fast, modular and extendable Monte-Carlo ray tracing engine written in Rust and OpenCL.
//!
//! You can find more information at the [Clay project website](https://clay-rs.github.io).

/// Reexport `clay-core`
pub use clay_core as core;

/// Mappings in render space.
pub mod map;
/// Material of an object.
pub mod material;
/// Shape of an object.
pub mod shape;

/// Scene to be rendered.
pub mod scene;
/// View of the scene.
pub mod view;

/// Filter for rendered image postprocessing.
pub mod filter;
/// Functionality for rendering pipeline.
pub mod process;
/// Loading the device OpenCL source code.
pub mod source;

/// Reexport of the basic traits.
pub mod prelude {
    pub use crate::core::prelude::*;
}

pub use clay_core::{Error, Result};

pub use clay_core::{buffer, context, object};

pub use context::*;
pub use prelude::*;
pub use source::*;

pub use clay_core::{
    instance_select, material_combine, material_select, object_select, shape_select,
};
