/*
 * Copyright 2018 German Research Center for Artificial Intelligence (DFKI)
 * Author: Clemens Lutz <clemens.lutz@dfki.de>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

mod bindings;
mod error;

use error::Result;

use std::ops::Drop;
use std::os::raw::{c_int, c_uint};
use std::ptr::null_mut;

/// Seed the random number generator before calling create_relation_xx. If not
/// called, then generator will be initialized with the time of the call which
/// produces different random numbers from run to run.
pub fn seed_generator(seed: u32) {
    unsafe { bindings::seed_generator(seed as c_uint) };
}

// TODO: support both KEY_8B and KEY_16B
pub struct Relation<'a>(&'a [bindings::tuple_t]);

pub enum BuildMode {
    Seq,
    Par(u32),
}

impl<'a> Relation<'a> {
    /// Create relation with non-unique keys uniformly distributed between [0, maxid]
    pub fn new_nonunique(len: i32, max_id: i32) -> Result<Self> {
        Self::safe_create(|r| unsafe { bindings::create_relation_nonunique(r, len, max_id) })
    }

    /// Create relation with only primary keys (i.e. keys are unique from 1 to
    /// num_tuples)
    ///
    /// If parallel mode is specified, creation procedure is executed by
    /// nthreads in parallel, where each memory is initialized thread local.
    pub fn new_pk(len: i32, mode: BuildMode) -> Result<Self> {
        match mode {
            BuildMode::Seq => {
                Self::safe_create(|r| unsafe { bindings::create_relation_pk(r, len) })
            }
            BuildMode::Par(threads) => Self::safe_create(|r| unsafe {
                bindings::parallel_create_relation_pk(r, len, threads)
            }),
        }
    }

    /// Create relation with foreign keys (i.e. duplicated keys exist). If ntuples is
    /// an exact multiple of maxid, (ntuples/maxid) sub-relations with shuffled keys
    /// following each other are generated.
    ///
    /// If parallel mode is specified, creation procedure is executed by
    /// nthreads in parallel, where each memory is initialized thread local.
    pub fn new_fk(len: i32, max_id: i32, mode: BuildMode) -> Result<Self> {
        match mode {
            BuildMode::Seq => {
                Self::safe_create(|r| unsafe { bindings::create_relation_fk(r, len, max_id) })
            }
            BuildMode::Par(threads) => Self::safe_create(|r| unsafe {
                bindings::parallel_create_relation_fk(r, len, max_id, threads)
            }),
        }
    }

    /// Create a foreign-key relation using the given primary-key relation and
    /// foreign-key relation size. If the keys in pkrel is randomly distributed in
    /// the full integer range, then
    pub fn new_fk_from_pk(pk: &Relation, len: i32) -> Result<Self> {
        let pk_r = bindings::relation_t {
            tuples: pk.0.as_ptr() as *mut bindings::tuple_t,
            num_tuples: pk.0.len() as u32,
        };

        Self::safe_create(|fk_r| unsafe {
            bindings::create_relation_fk_from_pk(
                fk_r,
                &pk_r as *const bindings::relation_t as *mut bindings::relation_t,
                len,
            )
        })
    }

    /// Create relation with keys distributed with zipf between [0, maxid]
    /// - zipf_param is the parameter of zipf distr (aka s)
    /// - maxid is equivalent to the alphabet size
    pub fn new_zipf(len: i32, max_id: i32, zipfparam: f64) -> Result<Self> {
        Self::safe_create(|r| unsafe { bindings::create_relation_zipf(r, len, max_id, zipfparam) })
    }

    fn safe_create<F>(f: F) -> Result<Self>
    where
        F: Fn(*mut bindings::relation_t) -> c_int,
    {
        let mut r = bindings::relation_t {
            tuples: null_mut(),
            num_tuples: 0,
        };

        let tuples = unsafe {
            if f(&mut r as *mut bindings::relation_t) != 0 {
                bail!("Couldn't create relation");
            }

            std::slice::from_raw_parts_mut(r.tuples, r.num_tuples as usize)
        };

        Ok(Relation(tuples))
    }
}

impl<'a> Drop for Relation<'a> {
    fn drop(&mut self) {
        let mut r = bindings::relation_t {
            tuples: self.0.as_ptr() as *mut bindings::tuple_t,
            num_tuples: self.0.len() as u32,
        };

        unsafe { bindings::delete_relation(&mut r as *mut bindings::relation_t) };
    }
}
