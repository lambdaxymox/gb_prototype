/*
 *  Googly Blocks is a video game.
 *  Copyright (C) 2018,2019,2020  Christopher Blanchard
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
use std::mem;


#[derive(Clone, Debug, PartialEq)]
pub struct Points {
    inner: Vec<[f32; 3]>,
}

impl Points {
    #[inline]
    pub fn as_ptr(&self) -> *const [f32; 3] {
        self.inner.as_ptr()
    }

    /// Get the length of the points buffer in bytes.
    #[inline]
    pub fn len_bytes(&self) -> usize {
        3 * mem::size_of::<f32>() * self.inner.len()
    }

    /// Get the number of elements in the points buffer.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextureCoordinates {
    inner: Vec<[f32; 2]>,
}

impl TextureCoordinates {
    #[inline]
    pub fn as_ptr(&self) -> *const [f32; 2] {
        self.inner.as_ptr()
    }

    /// Get the length of the texture coordinates buffer in bytes.
    #[inline]
    pub fn len_bytes(&self) -> usize {
        2 * mem::size_of::<f32>() * self.inner.len()
    }

    /// Get the number of elements in the texture coordinates buffer.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

/// An `ObjMesh` is a model space representation of a 2D geometric figure.
#[derive(Clone, Debug, PartialEq)]
pub struct Mesh {
    pub points: Points,
    pub tex_coords: TextureCoordinates,
}

impl Mesh {
    /// Generate a new mesh object.
    pub fn new(points: &[[f32; 3]], tex_coords: &[[f32; 2]]) -> Mesh {
        Mesh {
            points: Points { inner: points.iter().map(|e| *e).collect() },
            tex_coords: TextureCoordinates { inner: tex_coords.iter().map(|e| *e).collect() },
        }
    }

    /// Present the points map as an array slice. This function can be used
    /// to present the internal array buffer to OpenGL or another Graphics
    /// system for rendering.
    #[inline]
    pub fn points(&self) -> &[[f32; 3]] {
        &self.points.inner
    }

    /// Present the texture map as an array slice. This function can be used
    /// to present the internal array buffer to OpenGL or another Graphics
    /// system for rendering.
    #[inline]
    pub fn tex_coords(&self) -> &[[f32; 2]] {
        &self.tex_coords.inner
    }

    /// Get the number of vertices in the mesh.
    #[inline]
    pub fn len(&self) -> usize {
        self.points.len()
    }
}
