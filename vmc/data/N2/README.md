# N = 2

**Parameters without Jastrow**:

```rust
const ALPHA: f64 = 1.0;
const OMEGA: f64 = 1.0;
const BETA: f64 =  0.0;
const JASTROW: bool = false;
const STEP_SIZE: f64 = 0.1;
const MC_CYCLES: usize = 100_000;
const DIM: usize = 2;
const N: usize = 2;
const SPREAD: f64 = 0.1;
```

Total time spent: `65.874985369s`


**Parameters with Jastrow**:

```rust
const ALPHA: f64 = 1.0;
const OMEGA: f64 = 1.0;
const BETA: f64 =  0.0;
const JASTROW: bool = true;
const STEP_SIZE: f64 = 0.1;
const MC_CYCLES: usize = 100_000;
const DIM: usize = 2;
const N: usize = 2;
const SPREAD: f64 = 0.1;
```

Total time spent: `79.687830989s`
