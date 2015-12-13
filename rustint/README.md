###RustInt

#### Implementation of Big Integer library in Rust

A BigInt can be initialized as:

`let big_num = BigInt::new(34);
// where parameter to new method is the digit precision
`

or

`let big_num = BigInt::from_str("23432432423423");
`

Working operations as of now:

`Addition`

`let a = BigInt::from_str("324353453243234235345345634543423243432433453453435");`

`let b = BigInt::from_str("9934554345432423442423434534534534325");`

`let sum = a + b; //outputs : 324353453243244169899691066966865666866967987987760`


`
