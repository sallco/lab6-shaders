use lab6_shaders::obj::Obj;
use raylib::prelude::*;

#[test]
fn sphere_keeps_original_geometry_and_has_unit_normals() {
    let obj = Obj::load(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sphere.obj")).unwrap();
    assert_eq!(obj.vertices.len(), 98);
    assert_eq!(obj.indices.len(), 192 * 3);
    assert_eq!(obj.vertices[0], Vector3::new(0.5, 0.5, -0.5));
    for (position, normal) in obj.vertices.iter().zip(&obj.normals) {
        assert!((normal.length() - 1.0).abs() < 0.0001);
        assert!(normal.dot(*position) > 0.0);
    }
}

#[test]
fn invalid_faces_are_rejected_before_indexing() {
    let path = std::env::temp_dir().join(format!("lab6-invalid-{}.obj", std::process::id()));
    for face in ["f 0 2 3", "f 1 2 9", "f 1 2 3 1", "f -1 2 3"] {
        std::fs::write(&path, format!("v 0 0 0\nv 1 0 0\nv 0 1 0\n{face}\n")).unwrap();
        assert!(Obj::load(&path).is_err());
    }
    std::fs::write(&path, "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1/1/1 2//1 3/2/1").unwrap();
    assert_eq!(Obj::load(&path).unwrap().indices, [0, 1, 2]);
    std::fs::remove_file(path).unwrap();
}
