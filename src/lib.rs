// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

#[macro_export]
macro_rules! assert_approx_eq {
    ($left:expr, $right:expr) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val).approx_eq(&*right_val, None) {
                    panic!(
                        "assertion failed: `left.approx_eq(right)` \
                         (left: `{:?}`, right: `{:?}`)",
                        &*left_val, &*right_val
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr,) => {{ $crate::assert_approx_eq!($left, $right) }};
    ($left:expr, $right:expr, $max_diff:expr) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val).approx_eq(&*right_val, Some($max_diff)) {
                    panic!(
                        "assertion failed: `left.approx_eq(right)` \
                         (left: `{:?}`, right: `{:?}`)",
                        &*left_val, &*right_val
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr, $max_diff:expr,) => {{ $crate::assert_approx_eq!($left, $right, $max_diff) }};
}

#[macro_export]
macro_rules! debug_assert_approx_eq {
    ($left:expr, $right:expr) => {{
        if cfg!(debug_assertions) {
            $crate::assert_approx_eq!($left, $right)
        }
    }};
    ($left:expr, $right:expr,) => {{
        if cfg!(debug_assertions) {
            $crate::assert_approx_eq!($left, $right)
        }
    }};
    ($left:expr, $right:expr, $max_diff:expr) => {{
        if cfg!(debug_assertions) {
            $crate::assert_approx_eq!($left, $right, $max_diff: expr)
        }
    }};
    ($left:expr, $right:expr, $max_diff:expr,) => {{
        if cfg!(debug_assertions) {
            $crate::assert_approx_eq!($left, $right, $max_diff)
        }
    }};
}

#[macro_export]
macro_rules! assert_approx_ne {
    ($left:expr, $right:expr) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (*left_val).approx_eq(&*right_val, None) {
                    panic!(
                        "assertion failed: `left.approx_eq(right)` \
                         (left: `{:?}`, right: `{:?}`)",
                        &*left_val, &*right_val
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr,) => {{ $crate::assert_approx_ne!($left, $right) }};
    ($left:expr, $right:expr, $max_diff:expr) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (*left_val).approx_eq(&*right_val, Some($max_diff)) {
                    panic!(
                        "assertion failed: `left.approx_eq(right)` \
                         (left: `{:?}`, right: `{:?}`)",
                        &*left_val, &*right_val
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr, $max_diff:expr,) => {{ $crate::assert_approx_ne!($left, $right, $max_diff) }};
}

use num_traits::Float;

pub trait FloatApproxEq: Float + std::cmp::PartialEq + PartialOrd + Sized {
    const DEFAULT_MAX_DIFF: Self;

    fn abs_diff(&self, _other: &Self) -> Self;

    fn rel_diff_scale_factor(&self, _other: &Self) -> Self;

    fn approx_eq(&self, other: &Self, max_diff: Option<Self>) -> bool {
        if self.eq(other) {
            return true;
        }
        let max_diff = if let Some(max_diff) = max_diff {
            max_diff
        } else {
            Self::DEFAULT_MAX_DIFF
        };
        let abs_diff = self.abs_diff(other);
        if abs_diff <= max_diff {
            return true;
        }
        if abs_diff <= max_diff * self.rel_diff_scale_factor(other) {
            return true;
        }
        false
    }
}

impl FloatApproxEq for f64 {
    const DEFAULT_MAX_DIFF: Self = f64::EPSILON;

    fn abs_diff(&self, other: &Self) -> f64 {
        (self - other).abs()
    }

    fn rel_diff_scale_factor(&self, other: &Self) -> f64 {
        self.abs().max(other.abs())
    }
}

impl FloatApproxEq for f32 {
    const DEFAULT_MAX_DIFF: Self = f32::EPSILON;

    fn abs_diff(&self, other: &Self) -> f32 {
        (self - other).abs()
    }

    fn rel_diff_scale_factor(&self, other: &Self) -> f32 {
        self.abs().max(other.abs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f64_works() {
        assert_approx_eq!(
            10.0_f64 * 0.1,
            0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1
        );
        assert_ne!(
            10.0_f64 * 0.1,
            0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1
        );
    }

    #[test]
    fn f32_works() {
        assert_approx_eq!(
            10.0_f32 * 0.1,
            0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1
        );
        assert_ne!(
            10.0_f32 * 0.1,
            0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1
        );
    }
}
