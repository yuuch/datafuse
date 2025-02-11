// Copyright 2021 Datafuse Labs.
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

use std::sync::Arc;

use common_arrow::arrow::array::*;

use crate::prelude::*;

#[derive(Clone)]
pub struct StructColumn {
    values: Vec<ColumnRef>,
    data_type: DataTypePtr,
}

impl From<StructArray> for StructColumn {
    fn from(array: StructArray) -> Self {
        Self::new(array)
    }
}

impl StructColumn {
    pub fn new(array: StructArray) -> Self {
        let data_type = from_arrow_type(array.data_type());
        let values: Vec<ColumnRef> = array
            .values()
            .iter()
            .map(|v| v.clone().into_column())
            .collect();

        debug_assert!(!values.is_empty());
        Self { values, data_type }
    }

    pub fn from_arrow_array(array: &dyn Array) -> Self {
        Self::new(
            array
                .as_any()
                .downcast_ref::<StructArray>()
                .unwrap()
                .clone(),
        )
    }

    pub fn data_type(&self) -> DataTypePtr {
        self.data_type.clone()
    }

    pub fn from_data(values: Vec<ColumnRef>, data_type: DataTypePtr) -> Self {
        Self { values, data_type }
    }

    pub fn values(&self) -> &[ColumnRef] {
        &self.values
    }
}

impl Column for StructColumn {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn data_type(&self) -> DataTypePtr {
        self.data_type.clone()
    }

    fn len(&self) -> usize {
        self.values[0].len()
    }

    fn memory_size(&self) -> usize {
        self.values.iter().map(|v| v.memory_size()).sum()
    }

    fn as_arrow_array(&self) -> ArrayRef {
        let arrow_type = self.data_type().arrow_type();
        let arrays = self.values.iter().map(|v| v.as_arrow_array()).collect();
        Arc::new(StructArray::from_data(arrow_type, arrays, None))
    }

    fn slice(&self, offset: usize, length: usize) -> ColumnRef {
        let values = self
            .values
            .iter()
            .map(|v| v.slice(offset, length))
            .collect();

        Arc::new(Self {
            values,
            data_type: self.data_type.clone(),
        })
    }

    unsafe fn get_unchecked(&self, index: usize) -> DataValue {
        let values = self.values.iter().map(|v| v.get_unchecked(index)).collect();
        DataValue::Struct(values)
    }

    fn replicate(&self, offsets: &[usize]) -> ColumnRef {
        let values = self.values.iter().map(|v| v.replicate(offsets)).collect();

        Arc::new(Self {
            values,
            data_type: self.data_type.clone(),
        })
    }

    fn convert_full_column(&self) -> ColumnRef {
        Arc::new(self.clone())
    }
}
