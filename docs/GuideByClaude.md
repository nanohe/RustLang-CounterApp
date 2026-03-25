# Guide By Claude

I recommend building toward a multi-counter or task tracker. It is simple enough to stay focused, but each step naturally introduces an important Rust concept.

## Step-by-Step Progression

### Step 1: Structs and Methods

- Current counter app
- Clean up the dead comment
- Add `saturating_add` and `saturating_sub`

Teaches:
- Struct methods
- Basic state mutation
- Safer integer updates

### Step 2: Ownership and `String`

- Add a text input to name the counter

Teaches:
- `String` vs `&str`
- `.clone()`
- Owned data in structs

### Step 3: `Vec<T>` and Iteration

- Support multiple named counters
- Add create and remove actions

Teaches:
- `Vec<T>`
- `for` loops
- `.iter()`
- Index-based mutation

### Step 4: Enums and Pattern Matching

- Add counter categories such as `Daily`, `Weekly`, or `Unlimited`

Teaches:
- `enum`
- `match`
- `Option<T>`

### Step 5: Traits

- Implement `Display` for the counter type
- Derive `Debug`

Teaches:
- Traits
- `impl Trait for Type`
- Derive macros

### Step 6: Error Handling and File I/O

- Save and load counters from a JSON file using `serde`

Teaches:
- `Result<T, E>`
- The `?` operator
- External crates

### Step 7: Modules

- Split the code into `main.rs`, `app.rs`, and `counter.rs`

Teaches:
- `mod`
- `pub`
- File and module organization
