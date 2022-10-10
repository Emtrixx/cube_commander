use cgmath::{Vector3,InnerSpace};
use crate::{render::{primitives::Quad, vertex_types::DefaultVertex}, utils::colors::GREY};

#[derive(Clone)]
pub struct Shape {
    pub num_elements: u32,
    pub vertices: Vec<DefaultVertex>,
    pub indices: Vec<u32>,
}

impl Shape {
    pub fn new_default_cube(_name: String, _color: [f32; 3]) -> Self {
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
        let mut vertices: [DefaultVertex; 24] = [DefaultVertex {
            position: [1.0, -1.0, -1.0],
            color: GREY,
            normal: [1.0, 0.0, 0.0],
        }; 24];
        let mut indices: [u32; 36] = [0;36];

        for i in 0..quads.len() {
            for v in 0..quads[i].vertices.len() {
                vertices[(i*4)+v] = quads[i].vertices[v];
            }
            for p in 0..quads[i].indices.len() {
                indices[(i*6)+p] = quads[i].indices[p] + (i as u32 * 4);
            }
        }
        let num_indices = indices.len() as u32;

        Self {
           vertices: vertices.into(),
           indices: indices.into(),
           num_elements: num_indices
        }
    }
    pub fn new_plane(normal:[f32; 3], size: f32, color: [f32; 3]) -> Self{
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
        let num_elements = quad.indices.len() as u32;
        Shape {
            num_elements,
            vertices: quad.vertices.into(),
            indices: quad.indices.into(),
        }
    }
}