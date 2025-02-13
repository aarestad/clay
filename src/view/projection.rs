use crate::{prelude::*, view::View, Context};
use nalgebra::{Rotation3, Vector3};
use ocl::{self, builders::KernelBuilder, prm};
use std::collections::HashSet;

/// Perspective projection view.
#[derive(Debug, Clone)]
pub struct ProjectionView {
    /// Position of the point of view.
    pub pos: Vector3<f64>,
    /// Orientation.
    pub ori: Rotation3<f64>,
    /// Field of view width.
    pub fov: f64,
}

impl ProjectionView {
    pub fn new(pos: Vector3<f64>, ori: Rotation3<f64>) -> Self {
        Self { pos, ori, fov: 1.0 }
    }

    pub fn update(&mut self, pos: Vector3<f64>, ori: Rotation3<f64>) {
        self.pos = pos;
        self.ori = ori;
    }
}

impl View for ProjectionView {
    fn source(_: &mut HashSet<u64>) -> String {
        "#include <clay/view/proj_view.h>\n".to_string()
    }
}

impl Store for ProjectionView {
    type Data = Self;
    fn create_data(&self, _context: &Context) -> clay_core::Result<Self::Data> {
        Ok(self.clone())
    }
    fn update_data(&self, _context: &Context, data: &mut Self::Data) -> clay_core::Result<()> {
        *data = self.clone();
        Ok(())
    }
}

impl Push for ProjectionView {
    fn args_count() -> usize {
        3
    }
    fn args_def(kb: &mut KernelBuilder) {
        kb.arg(prm::Float3::zero())
            .arg(prm::Float16::zero())
            .arg(0.0f32);
    }
    fn args_set(&mut self, i: usize, k: &mut ocl::Kernel) -> crate::Result<()> {
        let mapf = self.ori.matrix().map(|x| x as f32);
        let mut map16 = [0f32; 16];
        map16[0..3].copy_from_slice(&mapf.as_slice()[0..3]);
        map16[4..7].copy_from_slice(&mapf.as_slice()[3..6]);
        map16[8..11].copy_from_slice(&mapf.as_slice()[6..9]);

        let posf = self.pos.map(|x| x as f32);
        let mut pos3 = [0f32; 3];
        pos3.copy_from_slice(posf.as_slice());

        k.set_arg(i + 0, &prm::Float3::from(pos3))?;
        k.set_arg(i + 1, &prm::Float16::from(map16))?;
        k.set_arg(i + 2, &(self.fov as f32))?;

        Ok(())
    }
}
