use crate::index::index::Index;
use crate::query_planner::query_planner::QueryPlanner;
use crate::types::{Filter, VectorElementType, SearchParams};

use crate::vector_storage::vector_storage::ScoredPointOffset;
use atomic_refcell::AtomicRefCell;
use std::sync::Arc;

pub struct SimpleQueryPlanner {
    index: Arc<AtomicRefCell<dyn Index>>
}

impl QueryPlanner for SimpleQueryPlanner {
    fn search(&self,
              vector: &Vec<VectorElementType>,
              filter: Option<&Filter>,
              top: usize,
              params: Option<&SearchParams>,
    ) -> Vec<ScoredPointOffset> {
        self.index.borrow().search(vector, filter, top, params)
    }
}

impl SimpleQueryPlanner {
    pub fn new(index: Arc<AtomicRefCell<dyn Index>>) -> Self {
        SimpleQueryPlanner {
            index
        }
    }
}