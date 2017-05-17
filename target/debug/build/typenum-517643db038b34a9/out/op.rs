
/**
Convenient type operations.

Any types representing values must be able to be expressed as `ident`s. That means they need to be
in scope.

For example, `P5` is okay, but `typenum::P5` is not.

You may combine operators arbitrarily.


# Example
```rust
#[macro_use] extern crate typenum;
use typenum::{P1, P2, P3, P4, P5, P10, N3, N7};

fn main() {
    type Result = cmp!(P10 == op!(min(P5 * (P3 + P4), (P1 - P2) * (N3 + N7))));
    use typenum::Bit;
    assert!(Result::to_bool());
}
```

The full list of supported operators is as follows. They all expand to type aliases defined in the
`operator_aliases` module.

---
Operator `*`. Expands to `Prod`.

```rust
# #[macro_use] extern crate typenum;
# use typenum::consts::*;
# use typenum::Bit;
# fn main() {
type Result = cmp!(P6 == op!(P3 * P2));    assert!(Result::to_bool());
# }
```

---
Operator `/`. Expands to `Quot`.

```rust
# #[macro_use] extern crate typenum;
# use typenum::consts::*;
# use typenum::Bit;
# fn main() {
type Result = cmp!(P3 == op!(P6 / P2));    assert!(Result::to_bool());
# }
```

---
Operator `%`. Expands to `Mod`.

```rust
# #[macro_use] extern crate typenum;
# use typenum::consts::*;
# use typenum::Bit;
# fn main() {
type Result = cmp!(P2 == op!(P5 % P3));    assert!(Result::to_bool());
# }
```

---
Operator `+`. Expands to `Sum`.

```rust
# #[macro_use] extern crate typenum;
# use typenum::consts::*;
# use typenum::Bit;
# fn main() {
type Result = cmp!(P5 == op!(P2 + P3));    assert!(Result::to_bool());
# }
```

---
Operator `-`. Expands to `Diff`.

```rust
# #[macro_use] extern crate typenum;
# use typenum::consts::*;
# use typenum::Bit;
# fn main() {
type Result = cmp!(N1 == op!(P2 - P3));    assert!(Result::to_bool());
# }
```

---
Operator `^`. Expands to `Xor`.

```rust
# #[macro_use] extern crate typenum;
# use typenum::consts::*;
# use typenum::Bit;
# fn main() {
type Result = cmp!(U6 == op!(U5 ^ U3));    assert!(Result::to_bool());
# }
```

---
Operator `|`. Expands to `Or`.

```rust
# #[macro_use] extern crate typenum;
# use typenum::consts::*;
# use typenum::Bit;
# fn main() {
type Result = cmp!(U7 == op!(U5 | U3));    assert!(Result::to_bool());
# }
```

---
Operator `&`. Expands to `And`.

```rust
# #[macro_use] extern crate typenum;
# use typenum::consts::*;
# use typenum::Bit;
# fn main() {
type Result = cmp!(U1 == op!(U5 & U3));    assert!(Result::to_bool());
# }
```

---
Operator `sqr`. Expands to `Square`.

```rust
# #[macro_use] extern crate typenum;
# use typenum::consts::*;
# use typenum::Bit;
# fn main() {
type Result = cmp!(P4 == op!(sqr(P2)));    assert!(Result::to_bool());
# }
```

---
Operator `cube`. Expands to `Cube`.

```rust
# #[macro_use] extern crate typenum;
# use typenum::consts::*;
# use typenum::Bit;
# fn main() {
type Result = cmp!(P8 == op!(cube(P2)));    assert!(Result::to_bool());
# }
```

---
Operator `pow`. Expands to `Exp`.

```rust
# #[macro_use] extern crate typenum;
# use typenum::consts::*;
# use typenum::Bit;
# fn main() {
type Result = cmp!(P8 == op!(pow(P2, P3)));    assert!(Result::to_bool());
# }
```

---
Operator `min`. Expands to `Minimum`.

```rust
# #[macro_use] extern crate typenum;
# use typenum::consts::*;
# use typenum::Bit;
# fn main() {
type Result = cmp!(P2 == op!(min(P2, P3)));    assert!(Result::to_bool());
# }
```

---
Operator `max`. Expands to `Maximum`.

```rust
# #[macro_use] extern crate typenum;
# use typenum::consts::*;
# use typenum::Bit;
# fn main() {
type Result = cmp!(P3 == op!(max(P2, P3)));    assert!(Result::to_bool());
# }
```

*/
#[macro_export]
macro_rules! op {
    ($($tail:tt)*) => ( __op_internal__!($($tail)*) );
}

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __op_internal__ {

(@stack[Prod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: * $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Prod, $($queue,)*] @tail: * $($tail)*)
);
(@stack[Quot, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: * $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Quot, $($queue,)*] @tail: * $($tail)*)
);
(@stack[Mod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: * $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Mod, $($queue,)*] @tail: * $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: * $($tail:tt)*) => (
    __op_internal__!(@stack[Prod, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[Prod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: / $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Prod, $($queue,)*] @tail: / $($tail)*)
);
(@stack[Quot, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: / $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Quot, $($queue,)*] @tail: / $($tail)*)
);
(@stack[Mod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: / $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Mod, $($queue,)*] @tail: / $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: / $($tail:tt)*) => (
    __op_internal__!(@stack[Quot, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[Prod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: % $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Prod, $($queue,)*] @tail: % $($tail)*)
);
(@stack[Quot, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: % $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Quot, $($queue,)*] @tail: % $($tail)*)
);
(@stack[Mod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: % $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Mod, $($queue,)*] @tail: % $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: % $($tail:tt)*) => (
    __op_internal__!(@stack[Mod, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[Prod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: + $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Prod, $($queue,)*] @tail: + $($tail)*)
);
(@stack[Quot, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: + $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Quot, $($queue,)*] @tail: + $($tail)*)
);
(@stack[Mod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: + $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Mod, $($queue,)*] @tail: + $($tail)*)
);
(@stack[Sum, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: + $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Sum, $($queue,)*] @tail: + $($tail)*)
);
(@stack[Diff, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: + $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Diff, $($queue,)*] @tail: + $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: + $($tail:tt)*) => (
    __op_internal__!(@stack[Sum, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[Prod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: - $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Prod, $($queue,)*] @tail: - $($tail)*)
);
(@stack[Quot, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: - $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Quot, $($queue,)*] @tail: - $($tail)*)
);
(@stack[Mod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: - $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Mod, $($queue,)*] @tail: - $($tail)*)
);
(@stack[Sum, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: - $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Sum, $($queue,)*] @tail: - $($tail)*)
);
(@stack[Diff, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: - $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Diff, $($queue,)*] @tail: - $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: - $($tail:tt)*) => (
    __op_internal__!(@stack[Diff, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[Prod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: ^ $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Prod, $($queue,)*] @tail: ^ $($tail)*)
);
(@stack[Quot, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: ^ $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Quot, $($queue,)*] @tail: ^ $($tail)*)
);
(@stack[Mod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: ^ $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Mod, $($queue,)*] @tail: ^ $($tail)*)
);
(@stack[Sum, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: ^ $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Sum, $($queue,)*] @tail: ^ $($tail)*)
);
(@stack[Diff, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: ^ $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Diff, $($queue,)*] @tail: ^ $($tail)*)
);
(@stack[Xor, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: ^ $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Xor, $($queue,)*] @tail: ^ $($tail)*)
);
(@stack[Or, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: ^ $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Or, $($queue,)*] @tail: ^ $($tail)*)
);
(@stack[And, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: ^ $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[And, $($queue,)*] @tail: ^ $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: ^ $($tail:tt)*) => (
    __op_internal__!(@stack[Xor, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[Prod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: | $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Prod, $($queue,)*] @tail: | $($tail)*)
);
(@stack[Quot, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: | $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Quot, $($queue,)*] @tail: | $($tail)*)
);
(@stack[Mod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: | $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Mod, $($queue,)*] @tail: | $($tail)*)
);
(@stack[Sum, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: | $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Sum, $($queue,)*] @tail: | $($tail)*)
);
(@stack[Diff, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: | $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Diff, $($queue,)*] @tail: | $($tail)*)
);
(@stack[Xor, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: | $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Xor, $($queue,)*] @tail: | $($tail)*)
);
(@stack[Or, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: | $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Or, $($queue,)*] @tail: | $($tail)*)
);
(@stack[And, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: | $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[And, $($queue,)*] @tail: | $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: | $($tail:tt)*) => (
    __op_internal__!(@stack[Or, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[Prod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: & $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Prod, $($queue,)*] @tail: & $($tail)*)
);
(@stack[Quot, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: & $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Quot, $($queue,)*] @tail: & $($tail)*)
);
(@stack[Mod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: & $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Mod, $($queue,)*] @tail: & $($tail)*)
);
(@stack[Sum, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: & $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Sum, $($queue,)*] @tail: & $($tail)*)
);
(@stack[Diff, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: & $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Diff, $($queue,)*] @tail: & $($tail)*)
);
(@stack[Xor, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: & $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Xor, $($queue,)*] @tail: & $($tail)*)
);
(@stack[Or, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: & $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Or, $($queue,)*] @tail: & $($tail)*)
);
(@stack[And, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: & $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[And, $($queue,)*] @tail: & $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: & $($tail:tt)*) => (
    __op_internal__!(@stack[And, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: sqr $($tail:tt)*) => (
    __op_internal__!(@stack[Square, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: cube $($tail:tt)*) => (
    __op_internal__!(@stack[Cube, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: pow $($tail:tt)*) => (
    __op_internal__!(@stack[Exp, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: min $($tail:tt)*) => (
    __op_internal__!(@stack[Minimum, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: max $($tail:tt)*) => (
    __op_internal__!(@stack[Maximum, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[LParen, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: , $($tail:tt)*) => (
    __op_internal__!(@stack[LParen, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[$stack_top:ident, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: , $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[$stack_top, $($queue,)*] @tail: , $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: ( $($stuff:tt)* ) $($tail:tt)* )
 => (
    __op_internal__!(@stack[LParen, $($stack,)*] @queue[$($queue,)*]
                     @tail: $($stuff)* RParen $($tail)*)
);
(@stack[LParen, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: RParen $($tail:tt)*) => (
    __op_internal__!(@rp3 @stack[$($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[$stack_top:ident, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: RParen $($tail:tt)*)
 => (
    __op_internal__!(@stack[$($stack,)*] @queue[$stack_top, $($queue,)*] @tail: RParen $($tail)*)
);
(@rp3 @stack[sqr, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: $($tail:tt)*) => (
    __op_internal__!(@rp3 @stack[$($stack,)*] @queue[Square, $($queue,)*] @tail: $($tail)*)
);
(@rp3 @stack[cube, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: $($tail:tt)*) => (
    __op_internal__!(@rp3 @stack[$($stack,)*] @queue[Cube, $($queue,)*] @tail: $($tail)*)
);
(@rp3 @stack[pow, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: $($tail:tt)*) => (
    __op_internal__!(@rp3 @stack[$($stack,)*] @queue[Exp, $($queue,)*] @tail: $($tail)*)
);
(@rp3 @stack[min, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: $($tail:tt)*) => (
    __op_internal__!(@rp3 @stack[$($stack,)*] @queue[Minimum, $($queue,)*] @tail: $($tail)*)
);
(@rp3 @stack[max, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: $($tail:tt)*) => (
    __op_internal__!(@rp3 @stack[$($stack,)*] @queue[Maximum, $($queue,)*] @tail: $($tail)*)
);
(@rp3 @stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: $num:ident $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[$num, $($queue,)*] @tail: $($tail)*)
);
(@stack[] @queue[$($queue:ident,)*] @tail: ) => (
    __op_internal__!(@reverse[] @input: $($queue,)*)
);
(@stack[$stack_top:ident, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail:) => (
    __op_internal__!(@stack[$($stack,)*] @queue[$stack_top, $($queue,)*] @tail: )
);
(@reverse[$($revved:ident,)*] @input: $head:ident, $($tail:ident,)* ) => (
    __op_internal__!(@reverse[$head, $($revved,)*] @input: $($tail,)*)
);
(@reverse[$($revved:ident,)*] @input: ) => (
    __op_internal__!(@eval @stack[] @input[$($revved,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Prod, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Prod<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Quot, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Quot<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Mod, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Mod<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Sum, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Sum<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Diff, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Diff<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Xor, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Xor<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Or, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Or<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[And, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::And<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Exp, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Exp<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Minimum, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Minimum<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Maximum, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Maximum<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $($stack:ty,)*] @input[Square, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Square<$a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $($stack:ty,)*] @input[Cube, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Cube<$a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$($stack:ty,)*] @input[$head:ident, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$head, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$stack:ty,] @input[]) => (
    $stack
);
($($tail:tt)* ) => (
    __op_internal__!(@stack[] @queue[] @tail: $($tail)*)
);
}