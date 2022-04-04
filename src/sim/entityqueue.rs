use std::collections::VecDeque;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueueEntry
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntityQueue
{
    // reference https://doc.rust-lang.org/std/collections/struct.VecDeque.html
    // using this so entries can be popped from the front
    queue: VecDeque<QueueEntry>
}

impl EntityQueue
{
    pub fn new() -> EntityQueue
    {
        EntityQueue
        {
            queue: VecDeque::<QueueEntry>::new(),
        }
    }
    // For getting the entity entry
    // whose turn it is
    pub fn pop(&mut self) -> Option<QueueEntry>
    {
        // could cause an Error if queue is empty and returns None
        self.queue.pop_front()
    }
    // Used for adding a new entry to queue based on action cost
    pub fn insert(&mut self, qe: QueueEntry)
    {
        let mut i = 0;
        if self.queue.is_empty()
        {
            self.queue.push(qe);

        } else
        {
            for entry in self.queue.iter()
            {
                if entry.get_action_cost() > qe.get_action_cost()
                {
                    break;
                }
                i = i + 1;
            }
            self.queue.insert(i,qe);
        }
    }
    // Used for removing entry if a action changes
    // or if an entity is removed
    pub fn remove_by_eid(&mut self, id: usize)
    {
        let mut i = 0;
        for entry in self.queue.iter()
        {
            if entry.get_eid() == id
            {
                self.queue.remove(i);
                break;
            }
            i = i + 1;
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    // QueueEntry Unit Tests
    //----------------------
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

    //Entity Queue Unit Tests
    //-----------------------
    #[test]
    fn can_pop_single_entry()
    {
        let mut q = EntityQueue::new();
        let qe = QueueEntry::new(11,200);
        q.insert(qe);
        let returned_value = q.pop().unwrap();
        assert_eq!(returned_value.get_eid(),11);
    }
    #[test]
    fn can_pop_multiple_entries()
    {
        let mut q = EntityQueue::new();
        let qe1 = QueueEntry::new(11,200);
        let qe2 = QueueEntry::new(12,200);
        let qe3 = QueueEntry::new(13,200);
        q.insert(qe1);
        q.insert(qe2);
        q.insert(qe3);
        q.pop();
        let returned_value = q.pop().unwrap();
        assert_eq!(returned_value.get_eid(),12);
    }
    #[test]
    fn pop_empty_queue()
    {
        let mut q = EntityQueue::new();
        assert!(q.pop().is_none());
    }
    #[test]
    fn insert_into_empty_queue()
    {
        let mut q = EntityQueue::new();
        let qe = QueueEntry::new(11,200);
        q.insert(qe);
        assert_eq!(q.queue.len(),1);
    }
    #[test]
    fn insert_different_action_into_nonempty_queue()
    {
        let mut q = EntityQueue::new();
        let qe1 = QueueEntry::new(11,200);
        let qe2 = QueueEntry::new(12,300);
        let qe3 = QueueEntry::new(13,200);
        q.insert(qe1);
        q.insert(qe2);
        q.insert(qe3);
        q.pop();
        q.pop();
        let returned_value = q.pop().unwrap();
        assert_eq!(returned_value.get_action_cost(),300);
    }
    #[test]
    fn insert_same_action_into_nonempty_queue()
    {
        let mut q = EntityQueue::new();
        let qe1 = QueueEntry::new(11,200);
        let qe2 = QueueEntry::new(12,200);
        let qe3 = QueueEntry::new(13,200);
        q.insert(qe1);
        q.insert(qe2);
        q.insert(qe3);
        q.pop();
        q.pop();
        let returned_value = q.pop().unwrap();
        assert_eq!(returned_value.get_eid(), 13);

    }
    #[test]
    fn can_remove()
    {
        let mut q = EntityQueue::new();
        let qe1 = QueueEntry::new(11,200);
        let qe2 = QueueEntry::new(12,200);
        let qe3 = QueueEntry::new(13,200);
        q.insert(qe1);
        q.insert(qe2);
        q.insert(qe3);
        q.remove_by_eid(12);
        q.pop();
        let returned_value = q.pop().unwrap();
        assert_ne!(returned_value.get_eid(), 12);
    }
    #[test]
    fn can_remove_nonexistant_entry()
    {
        let mut q = EntityQueue::new();
        let qe1 = QueueEntry::new(11,200);
        let qe2 = QueueEntry::new(14,200);
        let qe3 = QueueEntry::new(13,200);
        q.insert(qe1);
        q.insert(qe2);
        q.insert(qe3);
        q.remove_by_eid(12);
        q.pop();
        let returned_value = q.pop().unwrap();
        assert_eq!(returned_value.get_eid(), 14);
    }
    

}