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

use std::any::Any;
use std::sync::Arc;

use crate::ArrayColumn;
use crate::ArrayType;
use crate::ColumnRef;
use crate::DataTypePtr;
use crate::MutableColumn;

pub struct MutableArrayColumn<M: MutableColumn> {
    last_offset: usize,
    offsets: Vec<i64>,
    values: M,
    data_type: ArrayType,
}

impl<M: MutableColumn + 'static> MutableArrayColumn<M> {
    pub fn finish(&mut self) -> ArrayColumn {
        self.shrink_to_fit();
        self.last_offset = 0;
        let values = self.values.as_column();
        ArrayColumn {
            data_type: Arc::new(self.data_type.clone()),
            offsets: std::mem::take(&mut self.offsets).into(),
            values,
        }
    }
}

impl<M: MutableColumn + 'static> MutableColumn for MutableArrayColumn<M>
where M: MutableColumn
{
    fn data_type(&self) -> DataTypePtr {
        Arc::new(self.data_type.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn as_column(&mut self) -> ColumnRef {
        Arc::new(self.finish())
    }

    fn append_default(&mut self) {
        self.last_offset += 1;
        self.values.append_default();
        self.offsets.push(self.last_offset as i64);
    }

    fn shrink_to_fit(&mut self) {
        self.offsets.shrink_to_fit();
        self.values.shrink_to_fit();
    }
}
