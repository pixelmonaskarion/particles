use std::f32::consts::PI;

use cgmath::{vec3, Vector3};

use crate::{height_map::Vertex, sphere::{generate_sphere_smooth, generate_sphere_smooth_continued}};

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