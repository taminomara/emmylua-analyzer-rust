use std::collections::HashMap;

use rowan::{TextRange, TextSize};

#[derive(Debug, Clone)]
pub struct FileGenericIndex {
    generic_params: Vec<GenericParams>,
    root_node_ids: Vec<GenericEffectId>,
    effect_nodes: Vec<GenericEffectRangeNode>,
}

impl FileGenericIndex {
    pub fn new() -> Self {
        Self {
            generic_params: Vec::new(),
            root_node_ids: Vec::new(),
            effect_nodes: Vec::new(),
        }
    }

    pub fn add_generic_scope(&mut self, ranges: Vec<TextRange>, params: HashMap<String, usize>, is_func: bool) {
        let params_id = self.generic_params.len();
        self.generic_params.push(GenericParams::new(params, is_func));
        let params_id = GenericParamId::new(params_id);
        let root_node_ids: Vec<_> = self.root_node_ids.clone();
        for range in ranges {
            let mut added = false;
            for effect_id in root_node_ids.iter() {
                if self.try_add_range_to_effect_node(range, params_id, *effect_id) {
                    added = true;
                }
            }

            if !added {
                let child_node = GenericEffectRangeNode {
                    range,
                    params_id,
                    children: Vec::new(),
                };

                let child_node_id = self.effect_nodes.len();
                self.effect_nodes.push(child_node);
                self.root_node_ids.push(GenericEffectId::new(child_node_id));
            }
        }
    }

    fn try_add_range_to_effect_node(
        &mut self,
        range: TextRange,
        id: GenericParamId,
        effect_id: GenericEffectId,
    ) -> bool {
        let effect_node = self.effect_nodes.get(effect_id.id).unwrap();

        if effect_node.range.contains_range(range) {
            let children = effect_node.children.clone();
            for child_effect_id in children {
                if self.try_add_range_to_effect_node(range, id, child_effect_id) {
                    return true;
                }
            }

            let child_node = GenericEffectRangeNode {
                range,
                params_id: id,
                children: Vec::new(),
            };

            let child_node_id = self.effect_nodes.len();
            self.effect_nodes.push(child_node);
            let effect_node = self.effect_nodes.get_mut(effect_id.id).unwrap();
            effect_node
                .children
                .push(GenericEffectId::new(child_node_id));
            return true;
        }

        false
    }

    pub fn find_generic(&self, position: TextSize, name: &str) -> Option<(usize, bool)> {
        let params_id = self.find_generic_params(position)?;

        if let Some(params) = self.generic_params.get(params_id) {
            if let Some(id) = params.params.get(name) {
                return Some((*id, params.is_func));
            }
        }

        None
    }

    fn find_generic_params(&self, position: TextSize) -> Option<usize> {
        for effect_id in self.root_node_ids.iter() {
            if self
                .effect_nodes
                .get(effect_id.id)
                .unwrap()
                .range
                .contains(position)
            {
                return self.try_find_generic_params(position, *effect_id);
            }
        }

        None
    }

    fn try_find_generic_params(
        &self,
        position: TextSize,
        effect_id: GenericEffectId,
    ) -> Option<usize> {
        let effect_node = self.effect_nodes.get(effect_id.id).unwrap();
        for child_effect_id in effect_node.children.iter() {
            let child_effect_node = self.effect_nodes.get(child_effect_id.id).unwrap();
            if child_effect_node.range.contains(position) {
                return self.try_find_generic_params(position, *child_effect_id)
            }
        }

        Some(effect_node.params_id.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct GenericParamId {
    id: usize,
}

impl GenericParamId {
    fn new(id: usize) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GenericEffectRangeNode {
    range: TextRange,
    params_id: GenericParamId,
    children: Vec<GenericEffectId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct GenericEffectId {
    id: usize,
}

impl GenericEffectId {
    fn new(id: usize) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericParams {
    params: HashMap<String, usize>,
    is_func: bool
}

impl GenericParams {
    pub fn new(params: HashMap<String, usize>, is_func: bool) -> Self {
        Self {
            params,
            is_func
        }
    }
}