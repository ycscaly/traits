//! Elliptic curve arithmetic traits.

use crate::{
    ops::{Invert, LinearCombination, MulByGenerator, Reduce, ShrAssign},
    point::AffineCoordinates,
    scalar::{FromUintUnchecked, IsHigh},
    Curve, FieldBytes, PrimeCurve, ScalarPrimitive,
};
use core::fmt::Debug;
use subtle::{ConditionallySelectable, ConstantTimeEq, CtOption};
use zeroize::DefaultIsZeroes;

/// Elliptic curve with an arithmetic implementation.
pub trait CurveArithmetic: Curve {
    /// Elliptic curve point in affine coordinates.
    type AffinePoint: 'static
        + AffineCoordinates<FieldRepr = FieldBytes<Self>>
        + Copy
        + ConditionallySelectable
        + ConstantTimeEq
        + Debug
        + Default
        + DefaultIsZeroes
        + Eq
        + PartialEq
        + Sized
        + Send
        + Sync;

    /// Elliptic curve point in projective coordinates.
    ///
    /// Note: the following bounds are provided by [`group::Group`]:
    /// - `'static`
    /// - [`Copy`]
    /// - [`Clone`]
    /// - [`Debug`]
    /// - [`Eq`]
    /// - [`Sized`]
    /// - [`Send`]
    /// - [`Sync`]
    type ProjectivePoint: ConditionallySelectable
        + ConstantTimeEq
        + Default
        + DefaultIsZeroes
        + From<Self::AffinePoint>
        + Into<Self::AffinePoint>
        + LinearCombination
        + MulByGenerator
        + group::Curve<AffineRepr = Self::AffinePoint>
        + group::Group<Scalar = Self::Scalar>;

    /// Scalar field modulo this curve's order.
    ///
    /// Note: the following bounds are provided by [`ff::Field`]:
    /// - `'static`
    /// - [`Copy`]
    /// - [`Clone`]
    /// - [`ConditionallySelectable`]
    /// - [`ConstantTimeEq`]
    /// - [`Debug`]
    /// - [`Default`]
    /// - [`Send`]
    /// - [`Sync`]
    type Scalar: AsRef<Self::Scalar>
        + DefaultIsZeroes
        + From<ScalarPrimitive<Self>>
        + FromUintUnchecked<Uint = Self::Uint>
        + Into<FieldBytes<Self>>
        + Into<ScalarPrimitive<Self>>
        + Into<Self::Uint>
        + Invert<Output = CtOption<Self::Scalar>>
        + IsHigh
        + PartialOrd
        + Reduce<Self::Uint, Bytes = FieldBytes<Self>>
        + ShrAssign<usize>
        + ff::Field
        + ff::PrimeField<Repr = FieldBytes<Self>>;
}

/// Prime order elliptic curve with projective arithmetic implementation.
pub trait PrimeCurveArithmetic:
    PrimeCurve + CurveArithmetic<ProjectivePoint = Self::CurveGroup>
{
    /// Prime order elliptic curve group.
    type CurveGroup: group::prime::PrimeCurve<Affine = <Self as CurveArithmetic>::AffinePoint>;
}

/// Perform a batched conversion to affine representation on a sequence of projective points
/// at an amortized cost that should be practically as efficient as a single conversion.
/// Internally, implementors should rely upon `InvertBatch`.
pub trait ToAffineBatch: CurveArithmetic {
    /// Converts a batch of points in their projective representation into the affine ones.
    /// /// This variation takes a const-generic array and thus does not require `alloc`.
    fn to_affine_batch_array<const N: usize>(
        points: &[Self::ProjectivePoint; N],
    ) -> [Self::AffinePoint; N];

    /// Converts a batch of points in their projective representation into the affine ones.
    /// However, this also requires to make dynamic allocations and as such requires `alloc`.
    #[cfg(feature = "alloc")]
    fn to_affine_batch_slice<B: FromIterator<Self::AffinePoint>>(
        points: &[Self::ProjectivePoint],
    ) -> B;
}
