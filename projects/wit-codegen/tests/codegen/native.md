# <a name="native">World native</a>


 - Exports:
    - interface `v:native/number`
    - interface `v:native/text`

## <a name="v:native_number">Export interface v:native/number</a>

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
- <a name="integer.natural">`natural`</a>: [`Sign`](#sign)
----

### Functions

#### <a name="recast">`recast: func`</a>


##### Params

- <a name="recast.self">`self`</a>: [`Natural`](#natural)
- <a name="recast.rhs">`rhs`</a>: [`Natural`](#natural)

##### Return values

- <a name="recast.0"></a> [`Integer`](#integer)

#### <a name="new">`new: func`</a>


##### Params

- <a name="new.self">`self`</a>: [`Natural`](#natural)
- <a name="new.rhs">`rhs`</a>: [`Natural`](#natural)

##### Return values

- <a name="new.0"></a> [`Integer`](#integer)

## <a name="v:native_text">Export interface v:native/text</a>

----

### Types

#### <a name="natural">`type Natural`</a>
[`Natural`](#natural)
<p>
----

### Functions

#### <a name="recast">`recast: func`</a>


##### Params

- <a name="recast.self">`self`</a>: [`Natural`](#natural)
- <a name="recast.rhs">`rhs`</a>: [`Natural`](#natural)

##### Return values

- <a name="recast.0"></a> [`Integer`](#integer)

