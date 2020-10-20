use crate::sparse_set::SparseSet;
use crate::storage::EntityId;
use hashbrown::hash_set::HashSet;

/// Trait used as a bound for AllStorages::delete_any.
pub trait CustomDeleteAny {
    fn delete_any(&mut self, ids: &mut HashSet<EntityId>);
}

impl CustomDeleteAny for () {
    #[inline]
    fn delete_any(&mut self, _: &mut HashSet<EntityId>) {}
}

impl<T: 'static> CustomDeleteAny for SparseSet<T> {
    #[inline]
    fn delete_any(&mut self, ids: &mut HashSet<EntityId>) {
        ids.extend(&self.dense);
        self.clear();
    }
}
