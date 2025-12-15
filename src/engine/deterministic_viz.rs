use serde::{Serialize, Deserialize};

/// Deterministic visualization helpers for AxiomEngine
/// Provides a scene graph structure for organizing 3D objects

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    /// Root nodes in the scene graph
    pub nodes: Vec<SceneNode>,
    /// Ambient lighting configuration
    pub ambient_light: [f32; 3],
    /// Scene name/identifier
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneNode {
    /// Node identifier
    pub id: String,
    /// Node type (mesh, light, camera, etc.)
    pub node_type: NodeType,
    /// Transform (position, rotation, scale)
    pub transform: Transform,
    /// Child nodes
    pub children: Vec<SceneNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Mesh { mesh_id: String, material_id: String },
    Light { light_type: LightType, intensity: f32 },
    Camera { fov: f32, near: f32, far: f32 },
    Empty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LightType {
    Directional,
    Point,
    Spot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: [f32; 4], // Quaternion (x, y, z, w)
    pub scale: [f32; 3],
}

impl Scene {
    /// Create a new empty scene with default settings
    pub fn new() -> Self {
        Scene {
            nodes: Vec::new(),
            ambient_light: [0.2, 0.2, 0.2],
            name: String::from("DefaultScene"),
        }
    }

    /// Create a scene with a name
    pub fn with_name(name: impl Into<String>) -> Self {
        Scene {
            nodes: Vec::new(),
            ambient_light: [0.2, 0.2, 0.2],
            name: name.into(),
        }
    }

    /// Add a node to the scene
    pub fn add_node(&mut self, node: SceneNode) {
        self.nodes.push(node);
    }

    /// Set ambient lighting
    pub fn set_ambient_light(&mut self, r: f32, g: f32, b: f32) {
        self.ambient_light = [r, g, b];
    }

    /// Get all nodes of a specific type
    pub fn find_nodes_by_type(&self, node_type_filter: impl Fn(&NodeType) -> bool) -> Vec<&SceneNode> {
        let mut results = Vec::new();
        self.collect_nodes_recursive(&self.nodes, &node_type_filter, &mut results);
        results
    }

    fn collect_nodes_recursive<'a>(
        &'a self,
        nodes: &'a [SceneNode],
        filter: &impl Fn(&NodeType) -> bool,
        results: &mut Vec<&'a SceneNode>,
    ) {
        for node in nodes {
            if filter(&node.node_type) {
                results.push(node);
            }
            self.collect_nodes_recursive(&node.children, filter, results);
        }
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

impl Transform {
    /// Create identity transform
    pub fn identity() -> Self {
        Transform {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0], // Identity quaternion
            scale: [1.0, 1.0, 1.0],
        }
    }

    /// Create transform with position
    pub fn with_position(x: f32, y: f32, z: f32) -> Self {
        Transform {
            position: [x, y, z],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}

impl SceneNode {
    /// Create a new scene node
    pub fn new(id: impl Into<String>, node_type: NodeType) -> Self {
        SceneNode {
            id: id.into(),
            node_type,
            transform: Transform::identity(),
            children: Vec::new(),
        }
    }

    /// Add a child node
    pub fn add_child(&mut self, child: SceneNode) {
        self.children.push(child);
    }

    /// Set transform
    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
}
