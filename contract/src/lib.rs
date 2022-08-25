use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize,Debug,Clone)]
pub struct Task{
    id: u8,
    task: String,
    status: bool
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize,Debug)]
pub struct List {
    inner: HashMap<u8, Task>,
}

// Define the default, which automatically initializes the contract
impl Default for List {
    fn default() -> Self {
        Self { inner: HashMap::new() }
    }
}

// Implement the contract structure
impl List {
    pub fn add_task(&mut self, task_name:String)  {
        let task= Task{
            id:self.get_map_size()+1,
            task:task_name,
            status:false
        };
        self.inner.insert(task.id,task);
    }

    pub fn complete_task(&mut self, id:u8) {
        match self.inner.get_mut(&id){
            Some(value)=>{
                value.status=true
            }
            None =>()
        }
    }

    pub fn get_map_size(&self)->u8{    
        self.inner.len().try_into().unwrap()
    }

    pub fn get_all_tasks(&self)->Vec<Task>{
        self.inner.values().cloned().collect()
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;
    use std::vec::Vec;
    use std::any::type_name;

    fn same_type<T,Q>(_: T, _: Q) -> bool {
        type_name::<T>()==type_name::<Q>()
    }
    impl PartialEq for Task{
        fn eq(&self, other: &Self) -> bool {
            self.id ==other.id && self.task==other.task && self.status == other.status
        }
    }
    #[test]
    fn create_new_list() {
        let contract = List::default();
        let sample:HashMap<u8,Task> =HashMap::new();
        assert_eq!(true,same_type(contract.inner, sample));
    }
    
    #[test]
    fn check_size(){
        let mut list=List::default();
        list.add_task("do home work".to_owned());
        assert_eq!(list.get_map_size(),1)
    }
    
    #[test]
    fn check_update(){
        let mut list=List::default();
        list.add_task("do some thing".to_owned());
        list.add_task("do that".to_owned());
        list.complete_task(1);
        let task= Task { 
            id:1, 
            task: "do some thing".to_owned(), 
            status :true
        };
        if let Some(result)=list.inner.get(&1).cloned(){
            assert_eq!(result,task)
        };
    }

    #[test]
    fn check_get_values(){
        let mut arr=Vec::new();
        let task1=Task{
            id:1,
            task:"some thing".to_owned(),
            status:true
        };
        let task2=Task{
            id:2,
            task:"home work".to_owned(),
            status:true
        };
        arr.push(task1);
        arr.push(task2);

        let mut list=List::default();
        list.add_task("some thing".to_owned());
        list.add_task("home work".to_owned());
        list.complete_task(1);
        list.complete_task(2);
        let vec=list.get_all_tasks();
        let len=vec.len();
        for i in 0..len{
            assert_eq!(arr[i],vec[i]);
        }
    }
}