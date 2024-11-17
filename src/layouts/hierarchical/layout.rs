use std::collections::HashSet;

use egui::Pos2;
use petgraph::{
    csr::IndexType,
    stable_graph::NodeIndex,
    Direction::{Incoming, Outgoing},
    EdgeType,
};
use serde::{Deserialize, Serialize};

use crate::{
    layouts::{Layout, LayoutState},
    DisplayEdge, DisplayNode, Graph,
};

const ROW_DIST: usize = 50;
const NODE_DIST: usize = 50;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct State {}

impl LayoutState for State {}

#[derive(Debug, Default)]
pub struct Hierarchical {
    state: State,
}

impl Layout<State> for Hierarchical {
    fn next<N, E, Ty, Ix, Dn, De>(
        &mut self,
        g: &mut Graph<N, E, Ty, Ix, Dn, De>,
        not_placed: &HashSet<NodeIndex<Ix>>,
    ) where
        N: Clone,
        E: Clone,
        Ty: EdgeType,
        Ix: IndexType,
        Dn: DisplayNode<N, E, Ty, Ix>,
        De: DisplayEdge<N, E, Ty, Ix, Dn>,
    {
        let mut visited = HashSet::new();
        let mut max_col = 0;
        g.externals(Incoming)
            .collect::<Vec<NodeIndex<Ix>>>()
            .iter()
            .enumerate()
            .for_each(|(i, root_idx)| {
                visited.insert(*root_idx);

                let curr_max_col = build_tree(g, &mut visited, not_placed, root_idx, 0, i);
                if curr_max_col > max_col {
                    max_col = curr_max_col;
                };
            });
    }

    fn state(&self) -> State {
        self.state.clone()
    }

    fn from_state(state: State) -> impl Layout<State> {
        Hierarchical { state }
    }
}

fn build_tree<N, E, Ty, Ix, Dn, De>(
    g: &mut Graph<N, E, Ty, Ix, Dn, De>,
    visited: &mut HashSet<NodeIndex<Ix>>,
    not_placed: &HashSet<NodeIndex<Ix>>,
    root_idx: &NodeIndex<Ix>,
    start_row: usize,
    start_col: usize,
) -> usize
where
    N: Clone,
    E: Clone,
    Ty: EdgeType,
    Ix: IndexType,
    Dn: DisplayNode<N, E, Ty, Ix>,
    De: DisplayEdge<N, E, Ty, Ix, Dn>,
{
    let placed_idx = NodeIndex::new(root_idx.index());
    if !not_placed.contains(&placed_idx) {
        return start_col;
    };

    let y = start_row * ROW_DIST;
    let x = start_col * NODE_DIST;

    let node = g.node_mut(*root_idx).unwrap();
    node.set_location(Pos2::new(x as f32, y as f32));

    let mut max_col = start_col;
    g.neighbors_directed(*root_idx, Outgoing)
        .collect::<Vec<NodeIndex<Ix>>>()
        .iter()
        .enumerate()
        .for_each(|(i, neighbour_idx)| {
            if visited.contains(neighbour_idx) {
                return;
            };

            visited.insert(*neighbour_idx);

            let curr_max_col = build_tree(
                g,
                visited,
                not_placed,
                neighbour_idx,
                start_row + 1,
                start_col + i,
            );
            if curr_max_col > max_col {
                max_col = curr_max_col;
            };
        });

    max_col
}
