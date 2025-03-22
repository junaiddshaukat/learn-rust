# Rust Variables and Constants

## Variables in Rust

In Rust, variables are used to store values, but they have unique characteristics. Rust encourages safe programming by making variables **immutable by default**, though they can be explicitly made mutable if needed.

### 1. **Immutable Variables (Default Behavior)**
- Variables in Rust are immutable by default, meaning once a value is assigned to a variable, it cannot be changed.

**Example:**
```rust
let x = 10; // Immutable variable
println!("x: {}", x);
// x = 20; // This would cause an error because x is immutable.
```

### 2. **Mutable Variables**
- To create a variable whose value can be changed, use the `mut` keyword.

**Example:**
```rust
let mut a = 5; // 'mut' allows changing the variable value
println!("The value of a is: {a}");
a = 4; // This works because 'a' is mutable
println!("The updated value of a is: {a}");
```

### 3. **Constants**
- Constants are similar to immutable variables, but they:
  - Must be defined with an explicit type.
  - Are set using the `const` keyword.
  - Cannot be determined at runtime.
  - Can be declared globally (outside functions) and used anywhere.

**Example:**
```rust
// Global constant (defined outside main)
const P: i32 = 5;

fn main() {
    const THREE_HOURS_IN_SECONDS: i32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");
}
```

### Invalid Constant Usage:
```rust
// This would cause an error because constants must be determined at compile-time.
// const RUNTIME_CONSTANT: i32 = 60 * 60 * 3 + a; // Error
```

### 4. **Shadowing**
- Shadowing allows you to declare a variable with the same name multiple times.
- Unlike mutable variables, shadowing allows changing the **type** of the variable as well.
- Each `let` keyword creates a new variable binding, even if the name is the same.

**Example:**
```rust
fn main() {
    let abc = 32;
    println!("abc 1 : {abc}"); // Prints the integer value 32

    // Shadowing the variable with a new value (and changing its type)
    let abc = true;
    println!("abc 2 : {abc}"); // Prints the boolean value true
}
```

Shadowing differs from using `mut` because shadowing can change the type, while `mut` only allows changing the value within the same type.

---

## Full Rust Code Example
This example demonstrates immutable and mutable variables, constants, and shadowing:
```rust
// Variables and Constants in Rust

// Global constant (defined outside main)
const P: i32 = 5;

fn main() {
    // Example 1: Immutable variable (default behavior)
    let x = 10; // 'x' is immutable by default
    println!("The value of x is: {}", x);
    // x = 20; // This would cause an error because x is immutable.

    // Example 2: Mutable variable (can be changed)
    let mut a = 5; // 'mut' allows a variable to be changed
    println!("The value of a is: {a}");
    a = 4; // This works because 'a' is mutable
    println!("The updated value of a is: {a}");

    // Example 3: Using constants
    // Constants must be defined with an explicit type, and they cannot be changed.
    // Constants can also be used globally (outside the main function).
    const THREE_HOURS_IN_SECONDS: i32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");

    // Invalid constant usage (uncommenting the line below would cause an error)
    // because constants cannot be determined at runtime.
    // const RUNTIME_CONSTANT: i32 = 60 * 60 * 3 + a;

    // -----------------------------  Shadowing  ----------------------------------------------------

    // Shadowing allows reusing the same variable name with new values.
    // Unlike mutable variables, shadowing allows changing types as well.
    let abc = 32;
    println!("abc 1 : {abc}"); // Prints the integer value 32

    // Shadowing the variable with a new value (and even changing its type)
    let abc = true;
    println!("abc 2 : {abc}"); // Prints the boolean value true

    // This is different from using 'mut', as each new 'let' creates a new variable binding.
}
```