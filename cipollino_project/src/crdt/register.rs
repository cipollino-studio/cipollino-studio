
use std::{fmt::Debug, ops::Deref};

use super::time;

// #[derive(Clone)]
pub struct Register<T> {
    pub(crate) value: T,
    client_id: u64,
    last_write_client_id: u64,
    last_write_time: (u64, u64) 
}

impl<T: Debug> Debug for Register<T> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f) 
    }

}

impl<T> Register<T> {

    pub fn new(value: T, client_id: u64) -> Self {
        Self {
            value,
            client_id,
            last_write_client_id: client_id,
            last_write_time: time(),
        }
    }

    pub fn from_update(update: RegisterUpdate<T>, client_id: u64) -> Self {
        Self {
            value: update.value,
            client_id,
            last_write_client_id: update.client_id,
            last_write_time: update.time,
        }
    }

    pub(crate) fn set(&mut self, value: T) -> Option<RegisterUpdate<T>> where T: Clone {
        let time = time();
        if self.last_write_time > time {
            return None; 
        }
        if self.last_write_time == time && self.client_id > self.last_write_client_id {
            return None;
        }

        self.last_write_client_id = self.client_id;
        self.last_write_time = time;
        self.value = value.clone();
        Some(RegisterUpdate {
            value,
            client_id: self.client_id,
            time,
        }) 
    }

    pub(crate) fn apply(&mut self, update: RegisterUpdate<T>) -> bool {
        if update.time < self.last_write_time {
            return false;
        }
        if update.time == self.last_write_time && self.last_write_client_id < update.client_id {
            return false;
        }
        self.value = update.value;
        self.last_write_client_id = update.client_id;
        self.last_write_time = update.time;
        true
    }

    pub fn value(&self) -> &T {
        &self.value
    }

}

impl<T: Clone> Register<T> {

    pub fn to_update(&self) -> RegisterUpdate<T> {
        RegisterUpdate {
            value: self.value.clone(),
            client_id: self.client_id,
            time: self.last_write_time,
        }
    }

}



impl<T> Deref for Register<T> {

    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }

}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct RegisterUpdate<T> {
    pub value: T,
    client_id: u64,
    time: (u64, u64)
}

impl<T> Deref for RegisterUpdate<T> {

    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }

}
