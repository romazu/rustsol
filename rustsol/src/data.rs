pub const STORAGE_STRING: &str = r#"
{
  "storage": [
    {
      "astId": 2,
      "contract": "<stdin>:MyContract",
      "label": "plainUint112",
      "offset": 0,
      "slot": "0",
      "type": "t_uint112"
    },
    {
      "astId": 4,
      "contract": "<stdin>:MyContract",
      "label": "plainUint32",
      "offset": 14,
      "slot": "0",
      "type": "t_uint32"
    },
    {
      "astId": 6,
      "contract": "<stdin>:MyContract",
      "label": "plainString",
      "offset": 0,
      "slot": "1",
      "type": "t_string_storage"
    },
    {
      "astId": 14,
      "contract": "<stdin>:MyContract",
      "label": "myStruct",
      "offset": 0,
      "slot": "2",
      "type": "t_struct(MyStruct)11_storage"
    },
    {
      "astId": 18,
      "contract": "<stdin>:MyContract",
      "label": "staticArray",
      "offset": 0,
      "slot": "4",
      "type": "t_array(t_uint112)10_storage"
    },
    {
      "astId": 21,
      "contract": "<stdin>:MyContract",
      "label": "dynamicArray",
      "offset": 0,
      "slot": "9",
      "type": "t_array(t_uint256)dyn_storage"
    },
    {
      "astId": 25,
      "contract": "<stdin>:MyContract",
      "label": "myMapping",
      "offset": 0,
      "slot": "10",
      "type": "t_mapping(t_uint256,t_uint256)"
    },
    {
      "astId": 31,
      "contract": "<stdin>:MyContract",
      "label": "myNestedMapping",
      "offset": 0,
      "slot": "11",
      "type": "t_mapping(t_uint256,t_mapping(t_uint256,t_uint256))"
    }
  ],
  "types": {
    "t_address": {
      "encoding": "inplace",
      "label": "address",
      "numberOfBytes": "20"
    },
    "t_array(t_uint112)10_storage": {
      "base": "t_uint112",
      "encoding": "inplace",
      "label": "uint112[10]",
      "numberOfBytes": "160"
    },
    "t_array(t_uint256)dyn_storage": {
      "base": "t_uint256",
      "encoding": "dynamic_array",
      "label": "uint256[]",
      "numberOfBytes": "32"
    },
    "t_mapping(t_uint256,t_mapping(t_uint256,t_uint256))": {
      "encoding": "mapping",
      "key": "t_uint256",
      "label": "mapping(uint256 => mapping(uint256 => uint256))",
      "numberOfBytes": "32",
      "value": "t_mapping(t_uint256,t_uint256)"
    },
    "t_mapping(t_uint256,t_uint256)": {
      "encoding": "mapping",
      "key": "t_uint256",
      "label": "mapping(uint256 => uint256)",
      "numberOfBytes": "32",
      "value": "t_uint256"
    },
    "t_string_storage": {
      "encoding": "bytes",
      "label": "string",
      "numberOfBytes": "32"
    },
    "t_struct(MyStruct)11_storage": {
      "encoding": "inplace",
      "label": "struct MyContract.MyStruct",
      "members": [
        {
          "astId": 8,
          "contract": "<stdin>:MyContract",
          "label": "myAddress",
          "offset": 0,
          "slot": "0",
          "type": "t_address"
        },
        {
          "astId": 10,
          "contract": "<stdin>:MyContract",
          "label": "myUint",
          "offset": 0,
          "slot": "1",
          "type": "t_uint256"
        }
      ],
      "numberOfBytes": "64"
    },
    "t_uint112": {
      "encoding": "inplace",
      "label": "uint112",
      "numberOfBytes": "14"
    },
    "t_uint256": {
      "encoding": "inplace",
      "label": "uint256",
      "numberOfBytes": "32"
    },
    "t_uint32": {
      "encoding": "inplace",
      "label": "uint32",
      "numberOfBytes": "4"
    }
  }
}
"#;