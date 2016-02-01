// Copyright 2016 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(missing_docs)]

use {native, Resources};
use gfx_core::{draw, pso, shade, state, target, tex};
use gfx_core::{IndexType, VertexCount};
use winapi::{FLOAT, UINT8, D3D11_CLEAR_FLAG, D3D11_VIEWPORT};

///Serialized device command.
#[derive(Clone, Copy, Debug)]
pub enum Command {
    // states
    BindPixelTargets(pso::PixelTargetSet<Resources>),
    SetViewport(D3D11_VIEWPORT),
    // resource updates
    // drawing
    ClearColor(native::Rtv, [f32; 4]),
    ClearDepthStencil(native::Dsv, D3D11_CLEAR_FLAG, FLOAT, UINT8),
}

pub struct CommandBuffer {
    pub buf: Vec<Command>,
}

impl CommandBuffer {
    pub fn new() -> CommandBuffer {
        CommandBuffer {
            buf: Vec::new(),
        }
    }
}

impl draw::CommandBuffer<Resources> for CommandBuffer {
    fn clone_empty(&self) -> CommandBuffer { CommandBuffer::new() }
    fn reset(&mut self) {}
    fn bind_pipeline_state(&mut self, _: ()) {}
    fn bind_vertex_buffers(&mut self, _: pso::VertexBufferSet<Resources>) {}
    fn bind_constant_buffers(&mut self, _: pso::ConstantBufferSet<Resources>) {}
    fn bind_global_constant(&mut self, _: shade::Location, _: shade::UniformValue) {}
    fn bind_resource_views(&mut self, _: pso::ResourceViewSet<Resources>) {}
    fn bind_unordered_views(&mut self, _: pso::UnorderedViewSet<Resources>) {}
    fn bind_samplers(&mut self, _: pso::SamplerSet<Resources>) {}

    fn bind_pixel_targets(&mut self, pts: pso::PixelTargetSet<Resources>) {
        if let (Some(ref d), Some(ref s)) = (pts.depth, pts.stencil) {
            if d != s {
                error!("Depth and stencil views have to be the same");
            }
        }
        let viewport = D3D11_VIEWPORT {
            TopLeftX: 0.0, TopLeftY: 0.0,
            Width: pts.size.0 as f32, Height: pts.size.1 as f32,
            MinDepth: 0.0, MaxDepth: 1.0,
        };
        self.buf.push(Command::BindPixelTargets(pts));
        self.buf.push(Command::SetViewport(viewport));
    }

    fn bind_index(&mut self, _: ()) {}
    fn set_scissor(&mut self, _: Option<target::Rect>) {}
    fn set_ref_values(&mut self, _: state::RefValues) {}
    fn update_buffer(&mut self, _: (), _: draw::DataPointer, _: usize) {}
    fn update_texture(&mut self, _: native::Texture, _: tex::Kind, _: Option<tex::CubeFace>,
                      _: draw::DataPointer, _: tex::RawImageInfo) {}

    fn clear_color(&mut self, target: native::Rtv, value: draw::ClearColor) {
        match value {
            draw::ClearColor::Float(data) => {
                self.buf.push(Command::ClearColor(target, data));
            },
            _ => {
                error!("Unable to clear int/uint target");
            },
        }
    }

    fn clear_depth_stencil(&mut self, target: native::Dsv, depth: Option<target::Depth>,
                           stencil: Option<target::Stencil>) {
        let flags = //warning: magic constants ahead
            D3D11_CLEAR_FLAG(if depth.is_some() {1} else {0}) |
            D3D11_CLEAR_FLAG(if stencil.is_some() {2} else {0});
        self.buf.push(Command::ClearDepthStencil(target, flags,
                      depth.unwrap_or_default() as FLOAT,
                      stencil.unwrap_or_default() as UINT8
        ));
    }

    fn call_draw(&mut self, _: VertexCount, _: VertexCount, _: draw::InstanceOption) {}
    fn call_draw_indexed(&mut self, _: IndexType,
                         _: VertexCount, _: VertexCount,
                         _: VertexCount, _: draw::InstanceOption) {}
}
