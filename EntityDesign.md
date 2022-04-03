# Entites
Entities are the main agents in the simulation that are able to interact with each other and the
world around them.

## Entity Queue
-Vector QueueEntry

+ pop
+ insert
+ remove

### Queue Algorithm
1. Pop from Queue
2. Subtract Action Cost
3. Execute Action
4. Iterate over Entities to Check if target has changed
5. If target has changed -> Find new target
7. Choose next action
8. Loop  

### QueueEntry
- Entity ID
- Move Cost

+ get_eid
+ get_move_cost
+ update_move_cost

## Entity Decision Making
There are going to be 3 main types of entites each with similar but different decision making processes.

### Person

### Plant

### Animal

### Key events for entities
* Eating
* Reproduction
* Ageing
* Death
    * Drop items on death

### Entity Tags
* CAN_EAT
* CAN_MOVE
* CAN_SEE
* CAN_MOVE_THROUGH
* SEXUAL
* ASEXUAL
* CAN_PHOTOSYNTHESIZE