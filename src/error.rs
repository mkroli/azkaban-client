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

use std::fmt;

#[derive(Debug)]
pub enum AzkabanError {
    UrlParseError(::url::ParseError),
    HttpError(::hyper::error::Error),
    JsonDecoderError(::rustc_serialize::json::DecoderError),
    JsonParserError(::rustc_serialize::json::ParserError),
    MissingElementError(String),
    UnauthenticatedError
}

impl From<::url::ParseError> for AzkabanError {
    fn from(err: ::url::ParseError) -> Self {
        AzkabanError::UrlParseError(err)
    }
}

impl From<::hyper::error::Error> for AzkabanError {
    fn from(err: ::hyper::error::Error) -> Self {
        AzkabanError::HttpError(err)
    }
}

impl From<::rustc_serialize::json::DecoderError> for AzkabanError {
    fn from(err: ::rustc_serialize::json::DecoderError) -> Self {
        AzkabanError::JsonDecoderError(err)
    }
}

impl From<::rustc_serialize::json::ParserError> for AzkabanError {
    fn from(err: ::rustc_serialize::json::ParserError) -> Self {
        AzkabanError::JsonParserError(err)
    }
}

impl fmt::Display for AzkabanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &AzkabanError::UrlParseError(ref err) => err.fmt(f),
            &AzkabanError::HttpError(ref err) => err.fmt(f),
            &AzkabanError::JsonDecoderError(ref err) => err.fmt(f),
            &AzkabanError::JsonParserError(ref err) => err.fmt(f),
            &AzkabanError::MissingElementError(ref field) => write!(f, "JSON attribute {} not present", field),
            &AzkabanError::UnauthenticatedError => write!(f, "Not yet authenticated"),
        }
    }
}
