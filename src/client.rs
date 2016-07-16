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

use std::io;
use ::hyper::client::Client;
use ::hyper::header::{Headers, ContentType};
use ::hyper::mime::{Mime, TopLevel, SubLevel};
use ::url::Url;
use ::rustc_serialize::Decodable;
use ::rustc_serialize::json::Decoder;
use ::rustc_serialize::json::DecodeResult;
use ::rustc_serialize::json::Json;
use error::AzkabanError;
use response::{Executions, Flows, Jobs};

fn decode_from_reader<T: Decodable>(reader: &mut io::Read) -> DecodeResult<T> {
    let json = try!(Json::from_reader(reader));
    let mut decoder = Decoder::new(json);
    Ok(try!(Decodable::decode(&mut decoder)))
}

pub struct Azkaban {
    client: Client,
    base_url: Url,
    headers: Headers,
    session_id: Option<String>
}

impl Azkaban {
    pub fn new(base_url: &str) -> Result<Self, AzkabanError> {
        let client = Client::new();
        let url = try!(Url::parse(base_url));
        let mut headers = Headers::new();
        headers.set(
            ContentType(Mime(TopLevel::Application, SubLevel::WwwFormUrlEncoded, vec![]))
        );

        Ok(Azkaban {
            client: client,
            base_url: url,
            headers: headers,
            session_id: None
        })
    }

    pub fn authenticate(self: &mut Self, username: &str, password: &str) -> Result<(), AzkabanError> {
        let mut url = self.base_url.clone();
        url.query_pairs_mut().append_pair("action", "login");
        url.query_pairs_mut().append_pair("username", username);
        url.query_pairs_mut().append_pair("password", password);

        let mut response = try!(
            self.client.post(url.as_str())
                .headers(self.headers.clone())
                .send()
        );

        let json = try!(Json::from_reader(&mut response));

        let session_id = try!(
            json
                .as_object()
                .and_then(|o| { o.get("session.id") })
                .and_then(|o| { o.as_string() })
                .ok_or(AzkabanError::MissingElementError("session.id".to_string()))
        );

        self.session_id = Some(session_id.to_string());
        Ok(())
    }

    pub fn authenticated(base_url: &str, username: &str, password: &str) -> Result<Self, AzkabanError> {
        let mut azkaban = try!(Azkaban::new(base_url));
        try!(azkaban.authenticate(username, password));
        Ok(azkaban)
    }

    pub fn flows(self: &Self, project: &str) -> Result<Flows, AzkabanError> {
        let session_id = try!(self.session_id.clone().ok_or(AzkabanError::UnauthenticatedError));

        let mut url = self.base_url.clone();
        url.set_path("/manager");
        url.query_pairs_mut().append_pair("ajax", "fetchprojectflows");
        url.query_pairs_mut().append_pair("session.id", &session_id);
        url.query_pairs_mut().append_pair("project", project);

        let mut response = try!(
            self.client.get(url.as_str())
                .headers(self.headers.clone())
                .send()
        );

        let obj: Flows = try!(decode_from_reader(&mut response));

        Ok(obj)
    }

    pub fn jobs(self: &Self, project: &str, flow: &str) -> Result<Jobs, AzkabanError> {
        let session_id = try!(self.session_id.clone().ok_or(AzkabanError::UnauthenticatedError));

        let mut url = self.base_url.clone();
        url.set_path("/manager");
        url.query_pairs_mut().append_pair("ajax", "fetchflowgraph");
        url.query_pairs_mut().append_pair("session.id", &session_id);
        url.query_pairs_mut().append_pair("project", &project);
        url.query_pairs_mut().append_pair("flow", &flow);

        let mut response = try!(
            self.client.get(url.as_str())
                .headers(self.headers.clone())
                .send()
        );

        let obj: Jobs = try!(decode_from_reader(&mut response));

        Ok(obj)
    }

    pub fn executions(self: &Self, project: &str, flow: &str, start: u32, length: u32) -> Result<Executions, AzkabanError> {
        let session_id = try!(self.session_id.clone().ok_or(AzkabanError::UnauthenticatedError));

        let mut url = self.base_url.clone();
        url.set_path("/manager");
        url.query_pairs_mut().append_pair("ajax", "fetchFlowExecutions");
        url.query_pairs_mut().append_pair("session.id", &session_id);
        url.query_pairs_mut().append_pair("project", &project);
        url.query_pairs_mut().append_pair("flow", &flow);
        url.query_pairs_mut().append_pair("start", &start.to_string());
        url.query_pairs_mut().append_pair("length", &length.to_string());

        let mut response = try!(
            self.client.get(url.as_str())
                .headers(self.headers.clone())
                .send()
        );

        let obj: Executions = try!(decode_from_reader(&mut response));

        Ok(obj)
    }
}
