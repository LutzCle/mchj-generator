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

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate serde;

#[repr(C)]
#[derive(Debug, Serialize)]
pub struct tuple_t {
    pub key: intkey_t,
    pub payload: value_t,
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
