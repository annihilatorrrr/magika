// Copyright 2024 Google LLC
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

use std::sync::PoisonError;

use onnxruntime::OrtError;
use thiserror::Error;

/// Result type of Magika functions.
pub type MagikaResult<T> = Result<T, MagikaError>;

/// Errors returned by Magika functions.
#[derive(Debug, Error)]
pub enum MagikaError {
    /// Input/output errors reported by the standard library.
    #[error("I/O error")]
    IOError(#[from] std::io::Error),

    /// Errors reported by the ONNX Runtime.
    #[error("ONNX Runtime error")]
    OrtError(#[from] OrtError),

    /// Errors taking a lock on a mutex.
    #[error("Mutex lock error")]
    LockError,

    /// Errors reported by the JSON parser.
    #[error("JSON error")]
    JsonError(#[from] serde_json::Error),

    /// Shape errors reported by the ndarray library.
    #[error("ndarray shape error")]
    ShapeError(#[from] ndarray::ShapeError),
}

impl<T> From<PoisonError<T>> for MagikaError {
    fn from(_: PoisonError<T>) -> Self {
        MagikaError::LockError
    }
}
