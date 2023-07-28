## Warnings

**Warnings**: 6885

**Lines**: 43863

**KLOC**: 156.966

**Warning types**:

   3842 error: default numeric fallback might occur
   1919 error: arithmetic operation that can potentially result in unexpected side-effects
    806 error: unsafe block missing a safety comment
    160 error: type does not implement `std::fmt::Debug`; consider adding `#[derive(Debug)]` or a manual implementation
     56 error: used `expect()` on a `Result` value
     53 error: used `expect()` on an `Option` value
     22 error: docs for function which may panic missing `# Panics` section
      8 error: operator precedence can trip the unwary
      6 error: approximate value of `f64::consts::PI` found
      5 error: unsafe function's docs miss `# Safety` section
      3 error: used `unwrap()` on an `Option` value
      3 86 |     unsafe { while ferror(file) == 0 && feof(file) == 0 && fgetc(file) != '\n' as i32 {} }
      1 error: approximate value of `f64::consts::E` found
      1 81 |             error = sqrt(val * (1 as f64 - val) / sampled as f64) * 4 as f64;

## Runs statics

**Total runs**: 317

**Successful runs**: 278

**Failed runs**: 39

**Successful runs percentage**: 87.00%

**Success folders**:
```

```
**Failed folders**:
```

```
