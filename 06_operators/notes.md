# Rust Operators

## Arithmetic Operators

| Operator | Meaning | Example | Result |
|----------|---------|---------|--------|
| `+` | addition | `10 + 3` | `13` |
| `-` | subtraction | `10 - 3` | `7` |
| `*` | multiplication | `10 * 3` | `30` |
| `/` | division | `10 / 3` | `3` (integer division) |
| `%` | remainder | `10 % 3` | `1` |

> Float division keeps precision: `10.0 / 3.0 = 3.3333...`.

## Unary Minus

```rust
let x = 5;
let y = -x;   // -5
```

## Compound Assignment

```rust
let mut x = 10;
x += 5;   // 15
x -= 3;   // 12
x *= 2;   // 24
x /= 4;   // 6
x %= 4;   // 2
```

## Comparison Operators

| Operator | Meaning | Example | Result |
|----------|---------|---------|--------|
| `==` | equal | `10 == 3` | `false` |
| `!=` | not equal | `10 != 3` | `true` |
| `>` | greater than | `10 > 3` | `true` |
| `<` | less than | `10 < 3` | `false` |
| `>=` | greater or equal | `10 >= 10` | `true` |
| `<=` | less or equal | `10 <= 9` | `false` |

All comparison operators return `bool`.

## Logical Operators

| Operator | Meaning | Example | Result |
|----------|---------|---------|--------|
| `&&` | AND (short-circuit) | `true && false` | `false` |
| `\|\|` | OR (short-circuit) | `true \|\| false` | `true` |
| `!` | NOT | `!true` | `false` |

Short-circuit means the right side is only evaluated if needed:

```rust
if option.is_some() && option.unwrap() > 5 { ... }
// unwrap only runs if is_some() returned true
```

## Bitwise Operators

| Operator | Meaning | Example | Result |
|----------|---------|---------|--------|
| `&` | AND | `0b1100 & 0b1010` | `0b1000` |
| `\|` | OR | `0b1100 \| 0b1010` | `0b1110` |
| `^` | XOR | `0b1100 ^ 0b1010` | `0b0110` |
| `!` | NOT | `!0b1100` | all bits flipped |
| `<<` | left shift | `0b1100 << 2` | `0b110000` |
| `>>` | right shift | `0b1100 >> 1` | `0b0110` |

Bitwise compound assignment also works: `&=`, `|=`, `^=`, `<<=`, `>>=`.

## Range Operators

```rust
let a = 1..5;    // 1, 2, 3, 4   (exclusive)
let b = 1..=5;   // 1, 2, 3, 4, 5 (inclusive)
```

## Other Operators

| Operator | Meaning |
|----------|---------|
| `&` | borrow (create a reference) |
| `*` | dereference |
| `as` | type cast |
| `?` | error propagation (early return on `Err`) |
| `..` | range / struct update syntax |

## Operator Precedence (simplified, highest first)

1. `*` `/` `%`
2. `+` `-`
3. `<<` `>>`
4. `&`
5. `^`
6. `|`
7. `==` `!=` `<` `>` `<=` `>=`
8. `&&`
9. `||`
10. `=` `+=` `-=`, etc.

When in doubt, **use parentheses** — they cost nothing and make intent clear.

**Next:** [Booleans](../07_booleans) — `true` and `false`.
