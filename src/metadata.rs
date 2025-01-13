use egui::{Id, Pos2, Rect, Vec2};
use petgraph::{stable_graph::IndexType, EdgeType};
use serde::{Deserialize, Serialize};

use crate::{layouts::LayoutEvent, node_size, DisplayNode, Node};

const KEY: &str = "egui_graphs_metadata";

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Bounds {
    min: Vec2,
    max: Vec2,
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            min: Vec2::new(f32::MAX, f32::MAX),
            max: Vec2::new(f32::MIN, f32::MIN),
        }
    }
}

impl Bounds {
    pub fn compute_next<
        N: Clone,
        E: Clone,
        Ty: EdgeType,
        Ix: IndexType,
        D: DisplayNode<N, E, Ty, Ix>,
    >(
        &mut self,
        n: &Node<N, E, Ty, Ix, D>,
    ) {
        let size = node_size(n, Vec2::new(0., 1.));
        let loc = n.location();
        if loc.x + size < self.min.x {
            self.min.x = loc.x + size;
        };
        if loc.x + size > self.max.x {
            self.max.x = loc.x + size;
        };
        if loc.y - size < self.min.y {
            self.min.y = loc.y - size;
        };
        if loc.y + size > self.max.y {
            self.max.y = loc.y + size;
        };
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// Whether the frame is the first one
    pub first_frame: bool,
    /// Current zoom factor
    pub zoom: f32,
    /// Current pan offset
    pub pan: Vec2,
    /// Top left position of widget
    pub top_left: Pos2,

    pub last_layout_events: Option<Vec<LayoutEvent>>,

    /// State of bounds iteration
    bounds: Bounds,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            first_frame: true,
            zoom: 1.,
            pan: Vec2::default(),
            top_left: Pos2::default(),
            bounds: Bounds::default(),
            last_layout_events: None,
        }
    }
}

impl Metadata {
    pub fn load(ui: &egui::Ui) -> Self {
        ui.data_mut(|data| {
            data.get_persisted::<Metadata>(Id::new(KEY))
                .unwrap_or_default()
        })
    }

    pub fn save(self, ui: &mut egui::Ui) {
        ui.data_mut(|data| {
            data.insert_persisted(Id::new(KEY), self);
        });
    }

    pub fn canvas_to_screen_pos(&self, pos: Pos2) -> Pos2 {
        (pos.to_vec2() * self.zoom + self.pan).to_pos2()
    }

    pub fn canvas_to_screen_size(&self, size: f32) -> f32 {
        size * self.zoom
    }

    pub fn screen_to_canvas_pos(&self, pos: Pos2) -> Pos2 {
        ((pos.to_vec2() - self.pan) / self.zoom).to_pos2()
    }

    pub fn comp_iter_bounds<
        N: Clone,
        E: Clone,
        Ty: EdgeType,
        Ix: IndexType,
        D: DisplayNode<N, E, Ty, Ix>,
    >(
        &mut self,
        n: &Node<N, E, Ty, Ix, D>,
    ) {
        self.bounds.compute_next(n);
    }

    /// Returns bounding rect of the graph.
    pub fn graph_bounds(&self) -> Rect {
        Rect::from_min_max(self.bounds.min.to_pos2(), self.bounds.max.to_pos2())
    }

    /// Resets the bounds iterator.
    pub fn reset_bounds(&mut self) {
        self.bounds = Bounds::default();
    }
}
