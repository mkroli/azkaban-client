/*
 * Copyright 2016 Michael Krolikowski
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

extern crate rustc_serialize;

use ::rustc_serialize::Decodable;

#[derive(Debug)]
pub struct Jobs {
    pub project: String,
    pub project_id: i64,
    pub flow: String,
    pub nodes: Vec<Node>
}

impl Decodable for Jobs {
    fn decode<D: rustc_serialize::Decoder>(d: &mut D) -> Result<Jobs, D::Error> {
        d.read_struct("Jobs", 4, |d| {
            let project = try!(d.read_struct_field("project", 0, |d| { d.read_str() }));
            let project_id = try!(d.read_struct_field("projectId", 1, |d| { d.read_i64() }));
            let flow = try!(d.read_struct_field("flow", 2, |d| { d.read_str() }));
            let nodes = try!(d.read_struct_field("nodes", 3, |d| { Decodable::decode(d) }));
            Ok(Jobs {
                project: project,
                project_id: project_id,
                flow: flow,
                nodes: nodes
            })
        })
    }
}

#[derive(Debug)]
pub struct Node {
    pub id: String,
    pub job_type: String,
    pub job_dependencies: Vec<String>
}

impl Decodable for Node {
    fn decode<D: rustc_serialize::Decoder>(d: &mut D) -> Result<Node, D::Error> {
        d.read_struct("Node", 3, |d| {
            let id = try!(d.read_struct_field("id", 0, |d| { d.read_str() }));
            let job_type = try!(d.read_struct_field("type", 1, |d| { d.read_str() }));
            let job_dependencies = try!(
                d.read_struct_field("in", 2, |d| {
                    d.read_option(|d, set| {
                        if set {
                            d.read_seq(|d, len| {
                                let mut vec = Vec::with_capacity(len);
                                for i in 0..len {
                                    vec.push(try!(d.read_seq_elt(i, |d| {
                                        d.read_str()
                                    })));
                                }
                                Ok(vec)
                            })
                        } else {
                            Ok(vec!())
                        }
                    })
                })
            );
            Ok(Node {
                id: id,
                job_type: job_type,
                job_dependencies: job_dependencies
            })
        })
    }
}

#[derive(Debug)]
pub struct Executions {
    pub executions: Vec<Execution>,
    pub flow: String,
    pub from: i64,
    pub length: i64,
    pub project: String,
    pub project_id: i64,
    pub total: i64
}

impl Decodable for Executions {
    fn decode<D: rustc_serialize::Decoder>(d: &mut D) -> Result<Executions, D::Error> {
        d.read_struct("Executions", 7, |d| {
            let executions = try!(d.read_struct_field("executions", 0, |d| { Decodable::decode(d) }));
            let flow = try!(d.read_struct_field("flow", 1, |d| { d.read_str() }));
            let from = try!(d.read_struct_field("from", 2, |d| { d.read_i64() }));
            let length = try!(d.read_struct_field("length", 3, |d| { d.read_i64() }));
            let project = try!(d.read_struct_field("project", 4, |d| { d.read_str() }));
            let project_id = try!(d.read_struct_field("projectId", 5, |d| { d.read_i64() }));
            let total = try!(d.read_struct_field("total", 6, |d| { d.read_i64() }));
            Ok(Executions {
                executions: executions,
                flow: flow,
                from: from,
                length: length,
                project: project,
                project_id: project_id,
                total: total
            })
        })
    }
}

#[derive(Debug)]
pub struct Execution {
    pub end_time: i64,
    pub exec_id: i64,
    pub flow_id: String,
    pub project_id: i64,
    pub start_time: i64,
    pub status: String,
    pub submit_time: i64,
    pub submit_user: String
}

impl Decodable for Execution {
    fn decode<D: rustc_serialize::Decoder>(d: &mut D) -> Result<Execution, D::Error> {
        d.read_struct("Execution", 8, |d| {
            let end_time = try!(d.read_struct_field("endTime", 0, |d| { d.read_i64() }));
            let exec_id = try!(d.read_struct_field("execId", 1, |d| { d.read_i64() }));
            let flow_id = try!(d.read_struct_field("flowId", 2, |d| { d.read_str() }));
            let project_id = try!(d.read_struct_field("projectId", 3, |d| { d.read_i64() }));
            let start_time = try!(d.read_struct_field("startTime", 4, |d| { d.read_i64() }));
            let status = try!(d.read_struct_field("status", 5, |d| { d.read_str() }));
            let submit_time = try!(d.read_struct_field("submitTime", 6, |d| { d.read_i64() }));
            let submit_user = try!(d.read_struct_field("submitUser", 7, |d| { d.read_str() }));
            Ok(Execution {
                end_time: end_time,
                exec_id: exec_id,
                flow_id: flow_id,
                project_id: project_id,
                start_time: start_time,
                status: status,
                submit_time: submit_time,
                submit_user: submit_user
            })
        })
    }
}

#[derive(Debug)]
pub struct Flows {
    pub project: String,
    pub project_id: i64,
    pub flows: Vec<Flow>
}

impl Decodable for Flows {
    fn decode<D: rustc_serialize::Decoder>(d: &mut D) -> Result<Flows, D::Error> {
        d.read_struct("Flows", 3, |d| {
            let project = try!(d.read_struct_field("project", 0, |d| { d.read_str() }));
            let project_id = try!(d.read_struct_field("projectId", 1, |d| { d.read_i64() }));
            let flows = try!(d.read_struct_field("flows", 2, |d| { Decodable::decode(d) }));
            Ok(Flows {
                project: project,
                project_id: project_id,
                flows: flows
            })
        })
    }
}

#[derive(Debug)]
pub struct Flow {
    pub flow_id: String
}

impl Decodable for Flow {
    fn decode<D: rustc_serialize::Decoder>(d: &mut D) -> Result<Flow, D::Error> {
        d.read_struct("Flow", 1, |d| {
            let flow_id = try!(d.read_struct_field("flowId", 0, |d| { d.read_str() }));
            Ok(Flow {
                flow_id: flow_id
            })
        })
    }
}
