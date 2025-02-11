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

use common_datavalues2::prelude::*;
use common_exception::Result;
use pretty_assertions::assert_eq;

#[test]
fn test_serializers() -> Result<()> {
    struct Test {
        name: &'static str,
        data_type: DataTypePtr,
        value: DataValue,
        column: ColumnRef,
        val_str: &'static str,
        col_str: Vec<String>,
    }

    let tests = vec![
        Test {
            name: "boolean",
            data_type: BooleanType::arc(),
            value: DataValue::Boolean(true),
            column: Series::from_data(vec![true, false, true]),
            val_str: "1",
            col_str: vec!["1".to_owned(), "0".to_owned(), "1".to_owned()],
        },
        Test {
            name: "int8",
            data_type: Int8Type::arc(),
            value: DataValue::Int64(1),
            column: Series::from_data(vec![1i8, 2i8, 1]),
            val_str: "1",
            col_str: vec!["1".to_owned(), "2".to_owned(), "1".to_owned()],
        },
        Test {
            name: "datetime32",
            data_type: DateTimeType::arc(None),
            value: DataValue::UInt64(1630320462),
            column: Series::from_data(vec![1630320462u32, 1637117572u32, 1]),
            val_str: "2021-08-30 10:47:42",
            col_str: vec![
                "2021-08-30 10:47:42".to_owned(),
                "2021-11-17 02:52:52".to_owned(),
                "1970-01-01 00:00:01".to_owned(),
            ],
        },
        Test {
            name: "date32",
            data_type: Date32Type32::arc(),
            value: DataValue::Int64(18869),
            column: Series::from_data(vec![18869i32, 18948i32, 1]),
            val_str: "2021-08-30",
            col_str: vec![
                "2021-08-30".to_owned(),
                "2021-11-17".to_owned(),
                "1970-01-02".to_owned(),
            ],
        },
        Test {
            name: "string",
            data_type: StringType::arc(),
            value: DataValue::String("hello".as_bytes().to_vec()),
            column: Series::from_data(vec!["hello", "world", "NULL"]),
            val_str: "hello",
            col_str: vec!["hello".to_owned(), "world".to_owned(), "NULL".to_owned()],
        },
    ];

    for test in tests {
        let serializer = test.data_type.create_serializer();
        let val_res = serializer.serialize_value(&test.value)?;
        assert_eq!(&val_res, test.val_str, "case: {:#?}", test.name);

        let col_res = serializer.serialize_column(&test.column)?;
        assert_eq!(col_res, test.col_str, "case: {:#?}", test.name);
    }

    {
        let data_type = StructType::create(
            vec![
                "item_1".to_owned(),
                "item_2".to_owned(),
                "item_3".to_owned(),
                "item_4".to_owned(),
            ],
            vec![
                Float64Type::arc(),
                StringType::arc(),
                BooleanType::arc(),
                DateType::arc(),
            ],
        );
        let serializer = data_type.create_serializer();
        let value = DataValue::Struct(vec![
            DataValue::Float64(1.2),
            DataValue::String("hello".as_bytes().to_vec()),
            DataValue::Boolean(true),
            DataValue::UInt64(18869),
        ]);
        let result = serializer.serialize_value(&value)?;
        let expect = "(1.2, 'hello', 1, '2021-08-30')";
        assert_eq!(&result, expect);
    }

    Ok(())
}
