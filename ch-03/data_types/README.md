# Rust Data Types and Operations

This guide provides an overview of Rust's data types, including scalar and compound types, along with examples of how to use them in code. It also covers numeric operators and how to find the ranges of integer types.

## Scalar Types
Scalar types in Rust represent single values. Rust has four primary scalar types:

### 1. Integers
- **Signed Integers**: Allow negative and positive numbers (e.g., `i8`, `i32`).
- **Unsigned Integers**: Only allow positive numbers (e.g., `u8`, `u32`).

**Examples:**
```rust
let signed_num: i32 = -32;
let unsigned_num: u32 = 42;

let a = 23u8;    // u8 type
let b = 323_i32; // i32 type with visual separator
let num = 1_00_000; // 100000 with an underscore for readability
```

- By default, integers in Rust are of type `i32`.

**Integer Overflow:**
When compiling in debug mode, Rust will panic if an integer overflows. In release mode (`cargo run --release`), it starts a new cycle after reaching the limit.

**Example of Overflow:**
```rust
fn random() -> u8 {
    200
}
let a: u8 = random() + 100; // Causes integer overflow
println!("The value of a is {a}");
```

### 2. Floating-Point Numbers
Rust has two floating-point types:
- **f32**: Single precision
- **f64**: Double precision (default)

**Example:**
```rust
let float_default = 4.2;   // f64 by default
let float_single: f32 = 3.4; // f32 type
```

### 3. Booleans
Boolean type represents true or false values.

**Example:**
```rust
let tr = true;
let fal = false;
```

### 4. Characters
The `char` type represents a single character and supports Unicode.

**Example:**
```rust
let c = 'r';
let character: char = 'f'; // Explicit type annotation
let emoji = '😊';          // Unicode character
```

## Compound Types
Rust has two compound primitive types: **tuples** and **arrays**.

### Tuples
Tuples can group multiple values of different types. They have a fixed length.

**Example:**
```rust
let tuple = ("name", 25, true, 3.2);

// Destructuring a tuple
let (r, x, y, z) = tuple;
println!("The value of r is {r}, x is {x}, y is {y}, z is {z}");
```

### Arrays
Arrays are collections of values of the same type and have a fixed length.

**Example:**
```rust
let array = [1, 2, 3, 4, 5];
let typed_array: [i32; 3] = [10, 20, 30]; // Array with explicit type and length

// Accessing elements
let first = array[0];
let second = array[1];
println!("First element: {} and Second element: {}", first, second);
```

## Numeric Operators
Rust supports basic arithmetic operations:
- Addition: `+`
- Subtraction: `-`
- Multiplication: `*`
- Division: `/`
- Remainder: `%`

**Example:**
```rust
let add = 4 + 4;
let sub = 43 - 3;
let remainder = 5 % 2;
let float_div = 5.0 / 3.0; // Division with floating-point numbers
```

## Integer Ranges
To find the range of a data type in Rust, use the `MIN` and `MAX` constants.

**Example:**
```rust
fn main() {
    println!("i8 min: {}", i8::MIN);
    println!("i8 max: {}", i8::MAX);
    println!("u8 min: {}", u8::MIN);
    println!("u8 max: {}", u8::MAX);
    println!("i32 min: {}", i32::MIN);
    println!("i32 max: {}", i32::MAX);
    println!("u32 min: {}", u32::MIN);
    println!("u32 max: {}", u32::MAX);
}
```

### Integer Types and Their Ranges
#### Signed Integers
- **i8**: -128 to 127
- **i16**: -32,768 to 32,767
- **i32**: -2,147,483,648 to 2,147,483,647
- **i64**: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
- **i128**: -170,141,183,797,372,951,532,832,772,800 to 170,141,183,797,372,951,532,832,772,800
- **isize**: Depends on the architecture (e.g., 32-bit or 64-bit)

#### Unsigned Integers
- **u8**: 0 to 255
- **u16**: 0 to 65,535
- **u32**: 0 to 4,294,967,295
- **u64**: 0 to 18,446,744,073,709,551,615
- **u128**: 0 to 340,282,366,920,938,463,463,374,607,431,768,211,455
- **usize**: Depends on the architecture (e.g., 32-bit or 64-bit)

### Floating-Point Types
- **f32**: Single-precision floating-point (IEEE 754)
- **f64**: Double-precision floating-point (IEEE 754)

## Other Data Types
- **char**: Unicode scalar value (U+0000 to U+10FFFF)
- **bool**: `true` or `false`
- **()**: The unit type, with a single value `()`
