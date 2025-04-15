
# Type System

Types:
* Boolean
* I8, I16, I32, I64
* U8, U16, U32, U64
* F32, F64
* Object Pointer
* String
* Binary
* Array
* Enum Variant
* Map (string-value) 

# Representation

All integers are little-endian

### Value Representations

Type            | First Byte             | Notes
--------------- | ---------------------- | ----------------------------------------------------------------------------
Positive int    | `0xxxxxxx`             |
Small string    | `100xxxxx`             | Low 5 bits indicate size, followed by UTF string data
Small array     | `1010xxxx`             | Low 4 bits indicate length, followed by array elements
Small map       | `1011xxxx`             | Low 4 bits indicate number of fields, followed by struct fields(alternating symbol, value) 
False           | `11000000`             |
True            | `11000001`             |
U8              | `11000010`             | 
U16             | `11000011`             | 
U32             | `11000100`             | 
U64             | `11000101`             | 
I8              | `11000110`             | 
I16             | `11000111`             | 
I32             | `11001000`             | 
I64             | `11001001`             | 
F32             | `11001010`             | 
F64             | `11001011`             | 
String U8       | `11001100`             | Followed by a U8 indicating size, then string data
String U16      | `11001101`             | Followed by a U16 indicating size, then string data
String U32      | `11001110`             | Followed by a U32 indicating size, then string data
Binary U8       | `11001111`             | Followed by a U8 indicating size, then binary data
Binary U16      | `11010000`             | Followed by a U16 indicating size, then binary data
Binary U32      | `11010001`             | Followed by a U32 indicating size, then binary data
Array U8        | `11010010`             | Followed by a U8 indicating size, then array elements 
Array U16       | `11010011`             | Followed by a U16 indicating size, then array elements 
Array U32       | `11010100`             | Followed by a U32 indicating size, then array elements 
Map U8          | `11010101`             | Followed by a U8 indicating number of fields, then struct fields 
Map U16         | `11010110`             | Followed by a U16 indicating number of fields, then struct fields 
Map U32         | `11010111`             | Followed by a U32 indicating number of fields, then struct fields 
Idx'd Unit Enum | `11011xxx`             | Low 3 bits indicate enum variant
Idx'd Enum      | `11100xxx`             | Low 3 bits indicate enum variant, followed by variant data 
Small Obj Ptr   | `11101xxx`             | Low 3 bits indicate object type, followed by U32 for the object key 
Large Obj Ptr   | `11110xxx`             | Low 3 bits indicate object type, followed by U64 for the object key 
Small Obj Ptr   | `11111001`             | Followed by U8 indicating object type, then U32 for the object key
Large Obj Ptr   | `11111010`             | Followed by U8 indicating object type, then U64 for the object key
Small Obj Ptr   | `11111011`             | Followed by U16 indicating object type, then U32 for the object key
Large Obj Ptr   | `11111100`             | Followed by U16 indicating object type, then U64 for the object key
Null Obj Ptr    | `11111101`             |
Named Unit Enum | `11111110`             | Followed by a symbol(name of enum variant)
Named Enum      | `11111111`             | Followed by symbol(name of enum variant), then variant data 

### Symbol Representations

Optimized string representation used for enum variant names and name field names.

Type            | First Byte             | Notes
--------------- | ---------------------- | ----------------------------------------------------------------------------
Small string    | `0xxxxxxx`             | Low 7 bits indicate string length, followed by string data 
String U8       | `10000000`             | Followed by a U8 indicating string length, then string data
String U16      | `10000001`             | Followed by a U16 indicating string length, then string data
String U32      | `10000010`             | Followed by a U32 indicating string length, then string data