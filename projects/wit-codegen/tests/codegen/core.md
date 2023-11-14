# <a name="core">World core</a>


 - Exports:
    - interface `v:core/number`
    - interface `v:core/text`

## <a name="v:core_number">Export interface v:core/number</a>

----

### Types

#### <a name="sign">`enum Sign`</a>


##### Enum Cases

- <a name="sign.no_sign">`no-sign`</a>
- <a name="sign.positive">`positive`</a>
- <a name="sign.negative">`negative`</a>
#### <a name="natural">`record Natural`</a>


##### Record Fields

- <a name="natural.gc">`gc`</a>: `u32`
#### <a name="integer">`record Integer`</a>


##### Record Fields

- <a name="integer.sign">`sign`</a>: [`sign`](#sign)
- <a name="integer.natural">`natural`</a>: [`natural`](#natural)
----

### Functions

#### <a name="nat_new_u64">`nat-new-u64: func`</a>


##### Params

- <a name="nat_new_u64.n">`n`</a>: `u64`

##### Return values

- <a name="nat_new_u64.0"></a> [`natural`](#natural)

#### <a name="nat_add_u64">`nat-add-u64: func`</a>


##### Params

- <a name="nat_add_u64.this">`this`</a>: [`natural`](#natural)
- <a name="nat_add_u64.rhs">`rhs`</a>: `u64`

##### Return values

- <a name="nat_add_u64.0"></a> [`natural`](#natural)

#### <a name="nat_add_u64_inplace">`nat-add-u64-inplace: func`</a>


##### Params

- <a name="nat_add_u64_inplace.this">`this`</a>: [`natural`](#natural)
- <a name="nat_add_u64_inplace.rhs">`rhs`</a>: `u64`

#### <a name="nat_add_nat">`nat-add-nat: func`</a>


##### Params

- <a name="nat_add_nat.this">`this`</a>: [`natural`](#natural)
- <a name="nat_add_nat.rhs">`rhs`</a>: [`natural`](#natural)

##### Return values

- <a name="nat_add_nat.0"></a> [`natural`](#natural)

#### <a name="nat_add_nat_inplace">`nat-add-nat-inplace: func`</a>


##### Params

- <a name="nat_add_nat_inplace.this">`this`</a>: [`natural`](#natural)
- <a name="nat_add_nat_inplace.rhs">`rhs`</a>: [`natural`](#natural)

#### <a name="int_add_u64">`int-add-u64: func`</a>


##### Params

- <a name="int_add_u64.this">`this`</a>: [`integer`](#integer)
- <a name="int_add_u64.rhs">`rhs`</a>: `u64`

##### Return values

- <a name="int_add_u64.0"></a> [`integer`](#integer)

#### <a name="int_add_nat">`int-add-nat: func`</a>


##### Params

- <a name="int_add_nat.this">`this`</a>: [`integer`](#integer)
- <a name="int_add_nat.rhs">`rhs`</a>: [`natural`](#natural)

##### Return values

- <a name="int_add_nat.0"></a> [`integer`](#integer)

#### <a name="int_add_int">`int-add-int: func`</a>


##### Params

- <a name="int_add_int.this">`this`</a>: [`integer`](#integer)
- <a name="int_add_int.rhs">`rhs`</a>: [`integer`](#integer)

##### Return values

- <a name="int_add_int.0"></a> [`integer`](#integer)

## <a name="v:core_text">Export interface v:core/text</a>

----

### Types

#### <a name="ascii">`record Ascii`</a>


##### Record Fields

- <a name="ascii.codepoint">`codepoint`</a>: `u8`
#### <a name="unicode">`record Unicode`</a>


##### Record Fields

- <a name="unicode.codepoint">`codepoint`</a>: `u32`
----

### Functions

#### <a name="ascii_add_u8">`ascii-add-u8: func`</a>


##### Params

- <a name="ascii_add_u8.this">`this`</a>: [`ascii`](#ascii)
- <a name="ascii_add_u8.rhs">`rhs`</a>: `u8`

##### Return values

- <a name="ascii_add_u8.0"></a> [`ascii`](#ascii)

#### <a name="unicode_add_u32">`unicode-add-u32: func`</a>


##### Params

- <a name="unicode_add_u32.this">`this`</a>: [`unicode`](#unicode)
- <a name="unicode_add_u32.rhs">`rhs`</a>: `u32`

##### Return values

- <a name="unicode_add_u32.0"></a> [`unicode`](#unicode)

