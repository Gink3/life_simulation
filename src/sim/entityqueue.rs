

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
    pub fn get_eid()
    {
        todo!();
    }
    pub fn get_action_cost()
    {
        todo!();
    }
    pub fn update_action_cost()
    {
        todo!();
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