# <a name="core">World core</a>


 - Exports:
    - interface `v:core/number`
    - interface `v:core/text`

## <a name="v:core_number">Export interface v:core/number</a>

----

### Types

#### <a name="sign">`enum Sign`</a>


##### Enum Cases

#### <a name="natural">`type Natural`</a>
[`Natural`](#natural)
<p>
#### <a name="integer">`record Integer`</a>


##### Record Fields

- <a name="integer.sign">`sign`</a>: [`Sign`](#sign)
- <a name="integer.natural">`natural`</a>: [`Natural`](#natural)
----

### Functions

#### <a name="natural.new_u64">`Natural.new_u64: func`</a>


##### Params

- <a name="natural.new_u64.n">`n`</a>: `u64`

##### Return values

- <a name="natural.new_u64.0"></a> [`Natural`](#natural)

#### <a name="natural.add_u64">`Natural.add_u64: func`</a>


##### Params

- <a name="natural.add_u64.self">`self`</a>: [`Natural`](#natural)
- <a name="natural.add_u64.rhs">`rhs`</a>: `u64`

##### Return values

- <a name="natural.add_u64.0"></a> [`Natural`](#natural)

#### <a name="natural.add_u64_inplace">`Natural.add_u64_inplace: func`</a>


##### Params

- <a name="natural.add_u64_inplace.self">`self`</a>: [`Natural`](#natural)
- <a name="natural.add_u64_inplace.rhs">`rhs`</a>: `u64`

#### <a name="natural.add_nat">`Natural.add_nat: func`</a>


##### Params

- <a name="natural.add_nat.self">`self`</a>: [`Natural`](#natural)
- <a name="natural.add_nat.rhs">`rhs`</a>: [`Natural`](#natural)

##### Return values

- <a name="natural.add_nat.0"></a> [`Natural`](#natural)

#### <a name="natural.add_nat_inplace">`Natural.add_nat_inplace: func`</a>


##### Params

- <a name="natural.add_nat_inplace.self">`self`</a>: [`Natural`](#natural)
- <a name="natural.add_nat_inplace.rhs">`rhs`</a>: [`Natural`](#natural)

## <a name="v:core_text">Export interface v:core/text</a>

----

### Types

#### <a name="utf8_text">`type UTF8Text`</a>
[`UTF8Text`](#utf8_text)
<p>
----

### Functions

#### <a name="recast">`recast: func`</a>


##### Params

- <a name="recast.self">`self`</a>: [`UTF8Text`](#utf8_text)
- <a name="recast.rhs">`rhs`</a>: [`UTF8Text`](#utf8_text)

##### Return values

- <a name="recast.0"></a> [`Integer`](#integer)

