

struct QueueEntry
{
    eid: usize,
    action_cost: usize,
}

impl QueueEntry
{
    pub fn new(id: usize, ac: usize) -> QueueEntry
    {
        QueueEntry
        {
            eid: id,
            action_cost: ac,
        }
    }
    pub fn get_eid(&self) -> usize
    {
        self.eid
    }
    pub fn get_action_cost(&self) -> usize
    {
        self.action_cost
    }
    pub fn update_action_cost(&mut self, modifier: usize)
    {
        self.action_cost = self.action_cost - modifier;
    }
}

struct EntityQueue
{
    queue: Vector<QueueEntry>
}

impl EntityQueue
{
    // For getting the entity entry
    // whose turn it is
    pub fn pop()
    {
        todo!();
    }
    // Used for adding a new entry to queue based on action cost
    pub fn insert()
    {
        todo!();
    }
    // Used for removing entry if a action changes
    // or if an entity is removed
    pub fn remove()
    {
        todo!();
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn can_get_eid()
    {
        let qe = QueueEntry::new(11,200);
        assert_eq!(qe.get_eid(),11);
    }
    #[test]
    fn can_get_action_cost()
    {
        let qe = QueueEntry::new(11,200);
        assert_eq!(qe.get_action_cost(),200);
    }
    #[test]
    fn can_update_action_cost()
    {
        let mut qe = QueueEntry::new(11,200);
        qe.update_action_cost(100);
        assert_eq!(qe.get_action_cost(),200);
    }
}