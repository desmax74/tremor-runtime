// Copyright 2018-2020, Wayfair GmbH
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//NOTE: error_chain
#![allow(deprecated)]
#![allow(missing_docs)]
#![allow(clippy::large_enum_variant)]

use crate::async_sink;
use actix;
use base64;
use chrono;
use elastic;
use error_chain::*;
use grok;
use hdrhistogram::{self, serialization as hdr_s};
use log4rs;
use rdkafka;
use rental;
use reqwest;
use rmp_serde;
use serde_json;
use serde_yaml;
use std;
use tremor_pipeline;
//use tremor_script::LineValue;
use url;

impl Clone for Error {
    fn clone(&self) -> Self {
        ErrorKind::ClonedError(format!("{}", self)).into()
    }
}

impl From<actix::MailboxError> for Error {
    fn from(m: actix::MailboxError) -> Self {
        ErrorKind::MailboxError(m).into()
    }
}

impl From<hdr_s::DeserializeError> for Error {
    fn from(e: hdr_s::DeserializeError) -> Self {
        Self::from(format!("{:?}", e))
    }
}

impl From<hdrhistogram::errors::CreationError> for Error {
    fn from(e: hdrhistogram::errors::CreationError) -> Self {
        Self::from(format!("{:?}", e))
    }
}

impl From<hdrhistogram::RecordError> for Error {
    fn from(e: hdrhistogram::RecordError) -> Self {
        Self::from(format!("{:?}", e))
    }
}

impl From<hdrhistogram::serialization::V2SerializeError> for Error {
    fn from(e: hdrhistogram::serialization::V2SerializeError) -> Self {
        Self::from(format!("{:?}", e))
    }
}

impl From<google_storage1::Error> for Error {
    fn from(e: google_storage1::Error) -> Self {
        Self::from(format!("{:?}", e))
    }
}

impl From<google_pubsub1::Error> for Error {
    fn from(e: google_pubsub1::Error) -> Self {
        Self::from(format!("{:?}", e))
    }
}

impl<T> From<crossbeam_channel::SendError<T>> for Error {
    fn from(e: crossbeam_channel::SendError<T>) -> Self {
        Self::from(format!("{:?}", e))
    }
}
impl From<crossbeam_channel::RecvError> for Error {
    fn from(e: crossbeam_channel::RecvError) -> Self {
        Self::from(format!("{:?}", e))
    }
}

impl<P> From<std::sync::PoisonError<P>> for Error {
    fn from(e: std::sync::PoisonError<P>) -> Self {
        Self::from(format!("Poison Error: {:?}", e))
    }
}

impl<F> From<rental::RentalError<F, Box<Vec<u8>>>> for Error {
    fn from(_e: rental::RentalError<F, Box<Vec<u8>>>) -> Self {
        Self::from("Rental Error".to_string())
    }
}

#[cfg(test)]
impl PartialEq for Error {
    fn eq(&self, _other: &Self) -> bool {
        // This might be Ok since we try to compare Result in tets
        false
    }
}

error_chain! {
    links {
        Script(tremor_script::errors::Error, tremor_script::errors::ErrorKind);
        Pipeline(tremor_pipeline::errors::Error, tremor_pipeline::errors::ErrorKind);
    }
    foreign_links {
        Base64Error(base64::DecodeError);
        YAMLError(serde_yaml::Error) #[doc = "Error during yaml parsing"];
        JSONError(simd_json::Error);
        SerdeError(serde_json::Error);
        Io(std::io::Error) #[cfg(unix)];
        SinkDequeueError(async_sink::SinkDequeueError);
        SinkEnqueueError(async_sink::SinkEnqueueError);
        HTTPClientError(reqwest::Error);
        FromUTF8Error(std::string::FromUtf8Error);
        UTF8Error(std::str::Utf8Error);
        ElasticError(elastic::Error);
        KafkaError(rdkafka::error::KafkaError);
        ParseIntError(std::num::ParseIntError);
        UrlParserError(url::ParseError);
        ParseFloatError(std::num::ParseFloatError);
        LoggerError(log4rs::Error);
        ChannelReceiveError(std::sync::mpsc::RecvError);
        MsgPackDecoderError(rmp_serde::decode::Error);
        MsgPackEncoderError(rmp_serde::encode::Error);
        GrokError(grok::Error);
        DateTimeParseError(chrono::ParseError);
        SnappyError(snap::Error);
        AddrParseError(std::net::AddrParseError);
        RegexError(regex::Error);
    }

    errors {
        MailboxError(e: actix::MailboxError) {
            description("Actix mailbox error")
                display("Actix mailbox error: {:?}", e)
        }
        UnknownOp(n: String, o: String) {
            description("Unknown operator")
                display("Unknown operator: {}::{}", n, o)
        }
        ArtifactNotFound(id: String) {
            description("The artifact was not found")
                display("The artifact was not found: {}", id)
        }
        PublishFailedAlreadyExists(key: String) {
            description("The published artefact already exists")
                display("The published {} already exists.", key)
        }

        UnpublishFailedDoesNotExist(key: String) {
            description("The unpublished artefact does not exist")
                display("The unpublished {} does not exist.", key)
        }
        UnpublishFailedNonZeroInstances(key: String) {
            description("The artefact has non-zero instances and cannot be unpublished")
                display("Cannot unpublish artefact {} which has non-zero instances.", key)
        }
        UnpublishFailedSystemArtefact(key: String) {
            description("The artefact is a system artefact and cannot be unpublished")
                display("Cannot unpublish system artefact {}.", key)
        }

        BindFailedAlreadyExists(key: String) {
            description("The binding already exists")
                display("The binding with the id {} already exists.", key)
        }

        BindFailedKeyNotExists(key: String) {
            description("Failed to bind non existand artefact")
                display("Failed to bind non existand {}.", key)
        }

        // TODO: Old errors, verify if needed
        ClonedError(t: String) {
            description("This is a cloned error we need to get rod of this")
                display("Cloned error: {}", t)
        }
        MissingSteps(t: String) {
            description("missing steps")
                display("missing steps in: '{}'", t)
        }

        BadOpConfig(e: String) {
            description("Operator config has a bad syntax")
                display("Operator config has a bad syntax: {}", e)
        }

        UnknownNamespace(n: String) {
            description("Unknown namespace")
                display("Unknown namespace: {}", n)
        }

        InvalidGELFHeader(len: usize, initial: Option<[u8; 2]>) {
            description("Invalid GELF header")
                display("Invalid GELF header len: {}, prefix: {:?}", len, initial)
        }

        InvalidStatsD {
            description("Invalid statsd metric")
                display("Invalid statsd metric")
        }


        UnknownSubPipeline(p: String) {
            description("Reference to unknown sub-pipeline")
                display("The sub-pipelines '{}' is not defined", p)
        }

        UnknownPipeline(p: String) {
            description("Reference to unknown pipeline")
                display("The pipelines '{}' is not defined", p)
        }

        PipelineStartError(p: String) {
            description("Failed to start pipeline")
                display("Failed to start pipeline '{}'", p)
        }

        OnrampError(i: u64) {
            description("Error in onramp")
                display("Error in onramp '{}'", i)
        }

        OnrampMissingPipeline(o: String) {
            description("Onramp is missing a pipeline")
                display("The onramp '{}' is missing a pipeline", o)
        }
        InvalidInfluxData(s: String) {
            description("Invalid Influx Line Protocol data")
                display("Invalid Influx Line Protocol data: {}", s)
        }
        BadOutputid(i: usize) {
            description("Bad output pipeline id.")
                display("Bad output pipeline id {}", i - 1)
        }
        BadUTF8InString {
            description("Bad UTF8 in input string")

        }
        InvalidCompression {
            description("Data can't be decompressed")
                display("The data did not contain a known magic header to identify a supported compression")
        }
    }
}
