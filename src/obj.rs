use raylib::math::Vector3;
use std::{fs, io, path::Path};

pub struct Obj {
    pub vertices: Vec<Vector3>,
    pub indices: Vec<usize>,
    pub normals: Vec<Vector3>,
}

impl Obj {
    pub fn load(path: impl AsRef<Path>) -> io::Result<Self> {
        Self::parse(&fs::read_to_string(path)?)
    }

    fn parse(source: &str) -> io::Result<Self> {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        for (line_number, line) in source.lines().enumerate() {
            let mut fields = line.split('#').next().unwrap_or("").split_whitespace();
            let error = |message: &str| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Línea {}: {message}", line_number + 1),
                )
            };
            match fields.next() {
                Some("v") => {
                    let mut xyz = [0.0; 3];
                    for value in &mut xyz {
                        *value = fields
                            .next()
                            .ok_or_else(|| error("vértice incompleto"))?
                            .parse::<f32>()
                            .map_err(|_| error("coordenada inválida"))?;
                        if !value.is_finite() {
                            return Err(error("la coordenada debe ser finita"));
                        }
                    }
                    vertices.push(Vector3::new(xyz[0], xyz[1], xyz[2]));
                }
                Some("f") => {
                    let face: Vec<_> = fields.collect();
                    if face.len() != 3 {
                        return Err(error("se requieren caras trianguladas de tres índices"));
                    }
                    for field in face {
                        let index = field
                            .split('/')
                            .next()
                            .unwrap_or("")
                            .parse::<usize>()
                            .map_err(|_| error("se requieren índices positivos"))?;
                        indices.push(
                            index
                                .checked_sub(1)
                                .ok_or_else(|| error("el índice OBJ comienza en 1"))?,
                        );
                    }
                }
                _ => {}
            }
        }
        if vertices.is_empty() || indices.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "El OBJ debe contener vértices y triángulos",
            ));
        }
        if indices.iter().any(|&index| index >= vertices.len()) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Una cara referencia un vértice inexistente",
            ));
        }
        let mut normals = vec![Vector3::zero(); vertices.len()];
        for &[a, b, c] in indices.as_chunks::<3>().0 {
            let normal = (vertices[b] - vertices[a]).cross(vertices[c] - vertices[a]);
            for index in [a, b, c] {
                normals[index] += normal;
            }
        }
        for normal in &mut normals {
            if normal.length() < 1e-8 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "No se pudo calcular una normal",
                ));
            }
            *normal = normal.normalized();
        }
        Ok(Self {
            vertices,
            indices,
            normals,
        })
    }
}
