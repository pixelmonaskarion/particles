use std::f32::consts::PI;

use bespoke_engine::{binding::Descriptor, model::ToRaw};
use bytemuck::{bytes_of, NoUninit};
use cgmath::{vec3, Vector3};

use crate::sphere::{generate_sphere_smooth, generate_sphere_smooth_continued};

#[repr(C)]
#[derive(NoUninit, Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub normal: [f32; 3],
    // pub padding: [f32; 3],
}

impl Vertex {
    pub fn pos(&self) -> Vector3<f32> {
        return Vector3::new(self.position[0], self.position[1], self.position[2]);
    }
}

impl Descriptor for Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 9]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

impl ToRaw for Vertex {
    fn to_raw(&self) -> Vec<u8> {
        bytes_of(self).to_vec()
    }
}

#[derive(Debug, Clone)]
pub enum Atom {
    Carbon,
    Oxygen,
    Hydrogen,
}

impl Atom {
    pub fn size(&self) -> f32 {
        match self {
            Atom::Carbon => 0.67,
            Atom::Oxygen => 0.48,
            Atom::Hydrogen => 0.53,
            
        }
    }

    pub fn color(&self) -> [f32; 3] {
        match self {
            Atom::Carbon => [0., 0., 0.],
            Atom::Oxygen => [1., 0., 0.],
            Atom::Hydrogen => [1., 1., 1.],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Molecule {
    nodes: Vec<MoleculeNode>,
}

#[derive(Debug, Clone)]
pub struct MoleculeNode {
    atom: Atom,
    bonds: Vec<usize>,
}

pub fn carbon_dioxide() -> Molecule {
    return Molecule {
        nodes: vec![
            MoleculeNode {
                atom: Atom::Oxygen,
                bonds: vec![1],
            },
            MoleculeNode {
                atom: Atom::Carbon,
                bonds: vec![0, 2],
            },
            MoleculeNode {
                atom: Atom::Oxygen,
                bonds: vec![1],
            },
        ]
    };
}

pub fn CH2O() -> Molecule {
    return Molecule {
        nodes: vec![
            MoleculeNode {
                atom: Atom::Hydrogen,
                bonds: vec![1],
            },
            MoleculeNode {
                atom: Atom::Carbon,
                bonds: vec![0, 2, 3],
            },
            MoleculeNode {
                atom: Atom::Hydrogen,
                bonds: vec![1],
            },
            MoleculeNode {
                atom: Atom::Oxygen,
                bonds: vec![1],
            },
        ]
    };
}

pub fn H_ion() -> Molecule {
    return Molecule {
        nodes: vec![
            MoleculeNode {
                atom: Atom::Hydrogen,
                bonds: vec![]
            }
        ]
    }
}

pub fn generate_molecule_model(molecule: Molecule) -> (Vec<Vertex>, Vec<u32>) {
    let leaves = molecule.nodes.clone().into_iter().filter(|node| node.bonds.len() == 1).collect::<Vec<MoleculeNode>>();
    let center_atoms = molecule.nodes.clone().into_iter().filter(|node| node.bonds.len() != 1).collect::<Vec<MoleculeNode>>();
    if center_atoms.len() != 1 {
        panic!("I don't wanna code this");
    }
    let center_atom = center_atoms.first().unwrap();
    let (mut molecule_vertices, mut molecule_indices) = generate_sphere_smooth(center_atom.atom.size(), 4, 4, center_atom.atom.color());
    let shape = get_molecule_shape(center_atom.bonds.len());
    for (i, leaf) in leaves.iter().enumerate() {
        let length = center_atom.atom.size()+leaf.atom.size();
        let position = shape[i]*length;
        generate_sphere_smooth_continued(leaf.atom.size(), 4, 4, leaf.atom.color(), position.into(), &mut molecule_vertices, &mut molecule_indices);
    }
    (molecule_vertices, molecule_indices)
}

pub fn get_molecule_shape(bonds: usize) -> Vec<Vector3<f32>> {
    if bonds == 0 {
        return vec![]
    }
    if bonds == 1 {
        return vec![vec3(1., 0., 0.)]
    }
    if bonds == 2 {
        return vec![
            vec3(1., 0., 0.),
            vec3(-1., 0., 0.),
        ]
    }
    if bonds == 3 {
        return vec![
            vec3(1., 0., 0.),
            vec3((120. * PI / 180.).cos(), 0., (120. * PI / 180.).sin()),
            vec3((240. * PI / 180.).cos(), 0., (240. * PI / 180.).sin()),
        ]
    }
    panic!("I don't wanna code that");
}