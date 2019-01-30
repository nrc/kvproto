// Copyright 2019 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

use regex::Regex;
use std::env;
use std::fs::File;
use std::fs::{read_dir, remove_file};
use std::io::{Read, Write};
use std::process::Command;
use std::str;

fn main() {
    let buf_lib = BufferLib::from_env_vars();
    if buf_lib == BufferLib::Prost {
        unimplemented!("Prost support is not yet implemented");
    }

    check_protoc_version();

    let file_names: Vec<_> = read_dir("proto")
        .expect("Couldn't read proto directory")
        .map(|e| {
            format!(
                "proto/{}",
                e.expect("Couldn't list file").file_name().to_string_lossy()
            )
        })
        .collect();
    let file_names: Vec<_> = file_names.iter().map(|s| &**s).collect();

    for f in &file_names {
        println!("cargo:rerun-if-changed={}", f);
    }

    if buf_lib == BufferLib::Protobuf {
        generate_protobuf_files(file_names);
    }

    if buf_lib == BufferLib::Protobuf {
        let mod_names: Vec<_> = read_dir("src/protobuf")
            .expect("Couldn't read src directory")
            .filter_map(|e| {
                let file_name = e.expect("Couldn't list file").file_name();
                file_name
                    .to_string_lossy()
                    .split(".rs")
                    .next()
                    .map(|n| n.to_owned())
            })
            .collect();
        mod_names.sort();
        replace_read_unknown_fields(&mod_names);
        generate_protobuf_rs(&mod_names);
    }
}

#[derive(Eq, PartialEq)]
enum BufferLib {
    Prost,
    Protobuf,
}

impl BufferLib {
    fn from_env_vars() -> BufferLib {
        match (
            env::var_os("CARGO_FEATURE_PROST_BUF"),
            env::var_os("CARGO_FEATURE_PROTO_BUF"),
        ) {
            (Some(_), Some(_)) | (None, None) => {
                panic!("You must use exactly one of `proto-buf` and `prost-buf` features")
            }
            (Some(_), _) => BufferLib::Prost,
            (_, Some(_)) => BufferLib::Protobuf,
        }
    }
}

fn check_protoc_version() {
    let ver_re = Regex::new(r"([0-9]+)\.([0-9]+)\.[0-9]").unwrap();
    let ver = Command::new("protoc")
        .arg("--version")
        .output()
        .expect("Program `protoc` not installed (is it in PATH?).");
    let caps = ver_re
        .captures(str::from_utf8(&ver.stdout).unwrap())
        .unwrap();
    let major = caps.get(1).unwrap().as_str().parse::<i16>().unwrap();
    let minor = caps.get(2).unwrap().as_str().parse::<i16>().unwrap();
    if major == 3 && minor < 1 || major < 3 {
        panic!(
            "Invalid version of protoc (required 3.1.x, get {}.{}.x).",
            major, minor,
        );
    }
}

fn generate_protobuf_files(file_names: Vec<&str>) {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/protobuf",
        input: &file_names,
        includes: &["proto", "include"],
        customize: protoc_rust::Customize {
            ..Default::default()
        },
    })
    .unwrap();

    protoc_grpcio::compile_grpc_protos(file_names, &["proto", "include"], "src/protobuf").unwrap();
}

// Use the old way to read protobuf enums.
// FIXME: Remove this once stepancheg/rust-protobuf#233 is resolved.
fn replace_read_unknown_fields(mod_names: &[String]) {
    let regex =
        Regex::new(r"::protobuf::rt::read_proto3_enum_with_unknown_fields_into\(([^,]+), ([^,]+), &mut ([^,]+), [^\)]+\)\?").unwrap();
    for mod_name in mod_names {
        let file_name = &format!("src/protobuf/{}.rs", mod_name);

        let mut text = String::new();
        {
            let mut f = File::open(file_name).unwrap();
            f.read_to_string(&mut text)
                .expect("Couldn't read source file");
        }

        let text = regex.replace_all(
            &text,
            "if $1 == ::protobuf::wire_format::WireTypeVarint {\
                $3 = $2.read_enum()?;\
             } else {\
                return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));\
             }",
        );
        let mut out = File::create(file_name).unwrap();
        out.write_all(text.as_bytes())
            .expect("Could not write source file");
    }
}

fn generate_protobuf_rs(mod_names: &[String]) {
    let mut text = "use raft::eraftpb;\n\n".to_owned();

    for mod_name in mod_names {
        text.push_str("pub mod ");
        text.push_str(mod_name);
        text.push_str(";\n");
    }

    let mut lib = File::create("src/protobuf.rs").expect("Could not create protobuf.rs");
    lib.write_all(text.as_bytes())
        .expect("Could not write protobuf.rs");
}
