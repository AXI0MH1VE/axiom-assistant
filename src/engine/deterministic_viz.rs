// Deterministic visualization helpers for AxiomEngine
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Production-grade scene graph for deterministic rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    /// Root node of the scene graph
    root: SceneNode,
    /// Global scene metadata
    metadata: SceneMetadata,
    /// Named objects for quick lookup
    objects: HashMap<String, usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneNode {
    pub id: usize,
    pub name: String,
    pub transform: Transform,
    pub object_type: ObjectType,
    pub children: Vec<SceneNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: [f32; 4], // Quaternion [x, y, z, w]
    pub scale: [f32; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectType {
    Mesh { vertices: usize, indices: usize },
    Light { intensity: f32, color: [f32; 3] },
    Camera { fov: f32, near: f32, far: f32 },
    Empty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneMetadata {
    pub version: String,
    pub created_at: u64,
    pub deterministic_seed: u64,
}

impl Scene {
    /// Create a new empty scene with deterministic defaults
    pub fn new() -> Self {
        Self::with_seed(42)
    }

    /// Create scene with specific deterministic seed
    pub fn with_seed(seed: u64) -> Self {
        Scene {
            root: SceneNode {
                id: 0,
                name: "root".to_string(),
                transform: Transform::identity(),
                object_type: ObjectType::Empty,
                children: Vec::new(),
            },
            metadata: SceneMetadata {
                version: "1.0.0".to_string(),
                created_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
                deterministic_seed: seed,
            },
            objects: HashMap::new(),
        }
    }

    /// Add a node to the scene graph
    pub fn add_node(&mut self, name: String, parent_id: usize, object_type: ObjectType) -> Result<usize, String> {
        let new_id = self.objects.len() + 1;
        
        let new_node = SceneNode {
            id: new_id,
            name: name.clone(),
            transform: Transform::identity(),
            object_type,
            children: Vec::new(),
        };

        if parent_id == 0 {
            self.root.children.push(new_node);
        } else {
            self.add_node_recursive(&mut self.root, parent_id, new_node)?;
        }

        self.objects.insert(name, new_id);
        Ok(new_id)
    }

    fn add_node_recursive(&self, current: &mut SceneNode, parent_id: usize, node: SceneNode) -> Result<(), String> {
        if current.id == parent_id {
            current.children.push(node);
            return Ok(());
        }

        for child in &mut current.children {
            if let Ok(_) = self.add_node_recursive(child, parent_id, node.clone()) {
                return Ok(());
            }
        }

        Err(format!("Parent node {} not found", parent_id))
    }

    /// Serialize scene to JSON for deterministic rendering
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Load scene from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Get immutable reference to root node
    pub fn root(&self) -> &SceneNode {
        &self.root
    }

    /// Get node by name
    pub fn get_node_by_name(&self, name: &str) -> Option<usize> {
        self.objects.get(name).copied()
    }
}

impl Transform {
    /// Identity transform (no translation, rotation, or scale)
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
            ..Self::identity()
        }
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
