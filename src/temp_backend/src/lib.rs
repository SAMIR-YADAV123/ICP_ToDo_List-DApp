use ic_cdk::api::caller; 
use ic_cdk_macros::{query, update}; 
use ic_stable_structures::{StableBTreeMap, DefaultMemoryImpl, Storable}; // Stable storage structures
use serde::{Deserialize, Serialize}; // Serialization and deserialization
use candid::{CandidType, Principal}; 
use std::borrow::Cow;
use std::cell::RefCell; // Thread-local storage for managing state

// ask struct to represent a to-do item
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Task {
    pub id: u64,         
    pub title: String,  
    pub completed: bool, 
    pub owner: Principal, 
}

// Implement Storable trait for Task to enable storage in StableBTreeMap
impl Storable for Task {
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Unbounded;

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(serde_cbor::to_vec(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        serde_cbor::from_slice(&bytes).unwrap()
    }
}

// Thread-local storage for managing tasks and task IDs
thread_local! {
    
    static TASKS: RefCell<StableBTreeMap<u64, Task, DefaultMemoryImpl>> = RefCell::new(
        StableBTreeMap::init(DefaultMemoryImpl::default())
    );

    static NEXT_ID: RefCell<u64> = RefCell::new(0);
}

//To add a new task
#[update] 
pub fn add_task(title: String) -> u64 {
    let id = NEXT_ID.with(|next_id| {
        let mut id = next_id.borrow_mut();
        *id += 1; 
        *id
    });

    let task = Task {
        id,
        title,
        completed: false,
        owner: caller(), 
    };

    
    TASKS.with(|tasks| tasks.borrow_mut().insert(id, task));
    return id; 
}

// Function to retrieve all tasks
#[query] 
pub fn get_tasks() -> Vec<Task> {
    
    TASKS.with(|tasks| tasks.borrow().iter().map(|(_, task)| task.clone()).collect())
}

// Function to delete a task by ID of the task
#[update] 
pub fn delete_task(id: u64) -> Result<(), String> {
    TASKS.with(|tasks| {
        let mut tasks = tasks.borrow_mut();
        
        if tasks.remove(&id).is_some() {
            Ok(())
        } else {
            Err("Task not found".to_string())
        }
    })
}

// for updating the task if exists
#[update] 
pub fn update_task(id: u64, new_title: Option<String>, completed: Option<bool>) -> Result<(), String> {
    TASKS.with(|tasks| {
        let mut tasks = tasks.borrow_mut();
        
        // Check if the task exists
        if let Some(mut task) = tasks.get(&id) {
            
            if task.owner != caller() {
                return Err("You are not the owner of this task".to_string());
            }
            
          
            if let Some(title) = new_title {
                task.title = title;
            }

           
            if let Some(status) = completed {
                task.completed = status;
            }

            // Save the updated task back to TASKS
            tasks.insert(id, task);
            Ok(())
        } else {
            Err("Task not found".to_string())
        }
    })
}
