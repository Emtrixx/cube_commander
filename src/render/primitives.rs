use crate::render::model;
use crate::render::vertex_types::DefaultVertex;
use cgmath::InnerSpace;
use cgmath::Vector3;
use wgpu::Device;
use wgpu::util::DeviceExt;

// const RED: [f32; 3] = [1., 0.0, 0.0];
// const GREEN: [f32; 3] = [0., 1.0, 0.0];
// const BLUE: [f32; 3] = [0., 0.0, 1.0];
// const YELLOW: [f32; 3] = [1., 1.0, 1.0];
// const MAGENTA: [f32; 3] = [1., 0.0, 1.0];
// const CYAN: [f32; 3] = [0., 1.0, 1.0];
const GREY: [f32; 3] = [0.3, 0.3, 0.3];

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Quad {
    pub vertices: [DefaultVertex; 4],
    pub indices: [u32; 6],
}

pub struct Cube {
    pub quads: [Quad; 6],
    pub mesh: model::Mesh,
}

impl Cube {
    pub fn new(name: String, _color: [f32; 3], device: &wgpu::Device) -> Self {
        let quads = [
            // Back
            Quad {
                vertices: [
                    DefaultVertex {
                        position: [-1.0, -1.0, -1.0],
                        color: GREY,
                        normal: [0., 0., -1.0],
                    },
                    DefaultVertex {
                        position: [1.0, -1.0, -1.0],
                        color: GREY,
                        normal: [0., 0., -1.0],
                    },
                    DefaultVertex {
                        position: [-1.0, 1.0, -1.0],
                        color: GREY,
                        normal: [0., 0., -1.0],
                    },
                    DefaultVertex {
                        position: [1.0, 1.0, -1.0],
                        color: GREY,
                        normal: [0., 0., -1.0],
                    },
                ],
                indices: [0, 2, 1, 1, 2, 3],
            },
            // Top
            Quad {
                vertices: [
                    DefaultVertex {
                        position: [-1.0, 1.0, -1.0],
                        color: GREY,
                        normal: [0.0, 1.0, 0.0],
                    },
                    DefaultVertex {
                        position: [-1.0, 1.0, 1.0],
                        color: GREY,
                        normal: [0.0, 1.0, 0.0],
                    },
                    DefaultVertex {
                        position: [1.0, 1.0, -1.0],
                        color: GREY,
                        normal: [0.0, 1.0, 0.0],
                    },
                    DefaultVertex {
                        position: [1.0, 1.0, 1.0],
                        color: GREY,
                        normal: [0.0, 1.0, 0.0],
                    },
                ],
                indices: [0, 1, 2, 2, 1, 3],
            },
            // Front
            Quad {
                vertices: [
                    DefaultVertex {
                        position: [-1.0, -1.0, 1.0],
                        color: GREY,
                        normal: [0.0, 0.0, 1.0],
                    },
                    DefaultVertex {
                        position: [1.0, -1.0, 1.0],
                        color: GREY,
                        normal: [0.0, 0.0, 1.0],
                    },
                    DefaultVertex {
                        position: [-1.0, 1.0, 1.0],
                        color: GREY,
                        normal: [0.0, 0.0, 1.0],
                    },
                    DefaultVertex {
                        position: [1.0, 1.0, 1.0],
                        color: GREY,
                        normal: [0.0, 0.0, 1.0],
                    },
                ],
                indices: [0, 1, 2, 2, 1, 3],
            },
            // Bottom
            Quad {
                vertices: [
                    DefaultVertex {
                        position: [-1.0, -1.0, 1.0],
                        color: GREY,
                        normal: [0.0, -1.0, 0.0],
                    },
                    DefaultVertex {
                        position: [1.0, -1.0, 1.0],
                        color: GREY,
                        normal: [0.0, -1.0, 0.0],
                    },
                    DefaultVertex {
                        position: [-1.0, -1.0, -1.0],
                        color: GREY,
                        normal: [0.0, -1.0, 0.0],
                    },
                    DefaultVertex {
                        position: [1.0, -1.0, -1.0],
                        color: GREY,
                        normal: [0.0, -1.0, 0.0],
                    },
                ],
                indices: [0, 2, 3, 0, 3, 1],
            },
            // Left
            Quad {
                vertices: [
                    DefaultVertex {
                        position: [-1.0, -1.0, 1.0],
                        color: GREY,
                        normal: [-1.0, 0.0, 0.0],
                    },
                    DefaultVertex {
                        position: [-1.0, 1.0, 1.0],
                        color: GREY,
                        normal: [-1.0, 0.0, 0.0],
                    },
                    DefaultVertex {
                        position: [-1.0, 1.0, -1.0],
                        color: GREY,
                        normal: [-1.0, 0.0, 0.0],
                    },
                    DefaultVertex {
                        position: [-1.0, -1.0, -1.0],
                        color: GREY,
                        normal: [-1.0, 0.0, 0.0],
                    },
                ],
                indices: [0, 1, 2, 0, 2, 3],
            },
            // Right
            Quad {
                vertices: [
                    DefaultVertex {
                        position: [1.0, -1.0, 1.0],
                        color: GREY,
                        normal: [1.0, 0.0, 0.0],
                    },
                    DefaultVertex {
                        position: [1.0, 1.0, 1.0],
                        color: GREY,
                        normal: [1.0, 0.0, 0.0],
                    },
                    DefaultVertex {
                        position: [1.0, 1.0, -1.0],
                        color: GREY,
                        normal: [1.0, 0.0, 0.0],
                    },
                    DefaultVertex {
                        position: [1.0, -1.0, -1.0],
                        color: GREY,
                        normal: [1.0, 0.0, 0.0],
                    },
                ],
                indices: [0, 2, 1, 0, 3, 2],
            },
        ];
        let mut vertices: Vec<DefaultVertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        for (index, quad) in quads.iter().enumerate() {
            vertices.extend(quad.vertices);
            let n = quad.indices.clone().map(|f| f + (index as u32 * 4));
            indices.extend(n);
        }

        let num_indices = indices.len() as u32;

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Cube {
            mesh: model::Mesh {
                name,
                vertex_buffer,
                index_buffer,
                num_elements: num_indices,
            },
            quads,
        }
    }
}


pub struct Plane {
    pub size: f32,
    pub mesh: model::Mesh,
}

impl Plane {
    pub fn new(name: String, device: &Device, normal:[f32; 3], size: f32, color: [f32; 3]) -> Self{
        let up = Vector3::new(0.0, 1.0, 0.0);
        let dot = up.dot(normal.into());
        let p = up.cross(normal.into());
        let w = 1.0 + dot;
        let mut q = cgmath::Quaternion::new(p.x, p.y, p.z, w);
        if dot > 0.9999 {
            q = cgmath::Quaternion::new(1.0, 0.0, 0.0, 0.0);
        }

        let quad = Quad {
            vertices: [
                DefaultVertex {
                    position: (q * Vector3::new(1.0 * size, -2.0, 1.0 * size)).into(),
                    color,
                    normal,
                },
                DefaultVertex {
                    position: (q * Vector3::new(1.0 * size, -2.0, -1.0 * size)).into(),
                    color,
                    normal,
                },
                DefaultVertex {
                    position: (q * Vector3::new(-1.0 * size, -2.0, -1.0 * size)).into(),
                    color,
                    normal,
                },
                DefaultVertex {
                    position: (q * Vector3::new(-1.0 * size, -2.0, 1.0 * size)).into(),
                    color,
                    normal,
                },
            ],
            indices: [0,1,2,0,2,3],
        };

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&quad.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&quad.indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_elements = quad.indices.len() as u32;
        Plane {
            size,
            mesh: model::Mesh {
                name,
                vertex_buffer,
                index_buffer,
                num_elements,
            }
        }
    }
}