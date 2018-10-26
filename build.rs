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

extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;

fn main() {
    // Generate Rust bindings
    let bindings = bindgen::builder()
        .header("libmchj-generator/src/generator.h")
        .whitelist_recursively(false)
        .whitelist_type("intkey_t")
        .whitelist_type("relation_t")
        // .whitelist_type("tuple_t")
        .whitelist_type("value_t")
        .whitelist_function("seed_generator")
        .whitelist_function("create_relation_nonunique")
        .whitelist_function("create_relation_pk")
        .whitelist_function("create_relation_fk")
        .whitelist_function("create_relation_fk_from_pk")
        .whitelist_function("create_relation_zipf")
        .whitelist_function("parallel_create_relation_pk")
        .whitelist_function("parallel_create_relation_fk")
        // .whitelist_function("numa_localize")
        .whitelist_function("delete_relation")
        .generate_comments(true)
        .generate()
        .expect("Unable to generate MCHJ-generator bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write MCHJ-generator bindings");

    // Build C library
    let dst = cmake::build("libmchj-generator");

    // Link C library
    println!("cargo:rustc-link-lib=static=mchj-generator");
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
}
