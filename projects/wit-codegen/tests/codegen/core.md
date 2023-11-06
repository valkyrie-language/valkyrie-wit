# <a name="core">World core</a>


 - Exports:
    - interface `v:core/number`

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

#### <a name="integer">`record Integer`</a>


##### Record Fields

- <a name="integer.sign">`sign`</a>: [`sign`](#sign)
- <a name="integer.natural">`natural`</a>: [`natural`](#natural)
----

### Functions

#### <a name="nat_new_u_64">`nat-new-u-64: func`</a>


##### Params

- <a name="nat_new_u_64.n">`n`</a>: `u64`

##### Return values

- <a name="nat_new_u_64.0"></a> [`natural`](#natural)

#### <a name="nat_add_u_64">`nat-add-u-64: func`</a>


##### Params

- <a name="nat_add_u_64.this">`this`</a>: [`natural`](#natural)
- <a name="nat_add_u_64.rhs">`rhs`</a>: `u64`

##### Return values

- <a name="nat_add_u_64.0"></a> [`natural`](#natural)

#### <a name="nat_add_u_64_inplace">`nat-add-u-64-inplace: func`</a>


##### Params

- <a name="nat_add_u_64_inplace.this">`this`</a>: [`natural`](#natural)
- <a name="nat_add_u_64_inplace.rhs">`rhs`</a>: `u64`

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

#### <a name="int_add_u_64">`int-add-u-64: func`</a>


##### Params

- <a name="int_add_u_64.this">`this`</a>: [`integer`](#integer)
- <a name="int_add_u_64.rhs">`rhs`</a>: `u64`

##### Return values

- <a name="int_add_u_64.0"></a> [`integer`](#integer)

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

