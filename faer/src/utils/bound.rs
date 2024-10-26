use crate::{
    assert,
    hacks::GhostNode,
    variadics::{ArraySplit, DimSplit, SegmentSplit},
    Index, Shape, ShapeIdx, SignedIndex, Unbind,
};
use core::{fmt, marker::PhantomData, ops::Range};
use generativity::Guard;

type Invariant<'a> = fn(&'a ()) -> &'a ();
type Contravariant<'a> = fn(&'a ());

#[derive(Copy, Clone, Debug)]
pub struct Subset<'smol, 'big: 'smol>(PhantomData<&'smol &'big ()>);

pub trait ToLocal<'scope, 'dim, 'range>: Copy {
    type Local;
    fn to_local(range: Segment<'scope, 'dim, 'range>, this: Self) -> Self::Local;
}

pub trait ToGlobal<'scope, 'dim, 'range> {
    type Global;
    fn to_global(range: Segment<'scope, 'dim, 'range>, this: Self) -> Self::Global;
}

impl<'scope, 'dim, 'range> ToLocal<'scope, 'dim, 'range> for SegmentIdx<'scope, 'dim, 'range> {
    type Local = Idx<'range>;

    #[inline]
    fn to_local(range: Segment<'scope, 'dim, 'range>, this: Self) -> Self::Local {
        range.from_global(this)
    }
}

impl<'scope, 'dim, 'range> ToLocal<'scope, 'dim, 'range> for SegmentIdxInc<'scope, 'dim, 'range> {
    type Local = IdxInc<'range>;

    #[inline]
    fn to_local(range: Segment<'scope, 'dim, 'range>, this: Self) -> Self::Local {
        range.from_global_inc(this)
    }
}

impl<'scope, 'dim, 'range> ToGlobal<'scope, 'dim, 'range> for Idx<'range> {
    type Global = SegmentIdx<'scope, 'dim, 'range>;

    #[inline]
    fn to_global(range: Segment<'scope, 'dim, 'range>, this: Self) -> Self::Global {
        range.from_local(this)
    }
}

impl<'scope, 'dim, 'range> ToGlobal<'scope, 'dim, 'range> for IdxInc<'range> {
    type Global = SegmentIdxInc<'scope, 'dim, 'range>;

    #[inline]
    fn to_global(range: Segment<'scope, 'dim, 'range>, this: Self) -> Self::Global {
        range.from_local_inc(this)
    }
}

impl<'scope, 'dim, 'range: 'subrange, 'subrange> ToGlobal<'scope, 'dim, 'range>
    for SegmentIdx<'scope, 'dim, 'subrange>
{
    type Global = SegmentIdx<'scope, 'dim, 'range>;

    #[inline]
    fn to_global(range: Segment<'scope, 'dim, 'range>, this: Self) -> Self::Global {
        _ = range;
        this
    }
}

impl<'scope, 'dim, 'range: 'subrange, 'subrange> ToGlobal<'scope, 'dim, 'range>
    for SegmentIdxInc<'scope, 'dim, 'subrange>
{
    type Global = SegmentIdxInc<'scope, 'dim, 'range>;

    #[inline]
    fn to_global(range: Segment<'scope, 'dim, 'range>, this: Self) -> Self::Global {
        _ = range;
        this
    }
}

/// Splits a range into two segments.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Partition<'head, 'tail, 'n> {
    /// Size of the first half.
    pub head: Dim<'head>,
    /// Size of the second half.
    pub tail: Dim<'tail>,
    __marker: PhantomData<Invariant<'n>>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SegmentIdx<'scope, 'dim, 'range> {
    unbound: usize,
    __marker: PhantomData<(Invariant<'scope>, Invariant<'dim>, Contravariant<'range>)>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SegmentIdxInc<'scope, 'dim, 'range> {
    unbound: usize,
    __marker: PhantomData<(Invariant<'scope>, Invariant<'dim>, Contravariant<'range>)>,
}

impl core::fmt::Debug for SegmentIdx<'_, '_, '_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.unbound.fmt(f)
    }
}
impl core::fmt::Debug for SegmentIdxInc<'_, '_, '_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.unbound.fmt(f)
    }
}
impl core::fmt::Debug for Segment<'_, '_, '_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Segment")
            .field("start", &self.start)
            .field("end", &self.end)
            .field("len", &self.len())
            .finish()
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Segment<'scope, 'dim, 'range> {
    start: IdxInc<'dim>,
    end: IdxInc<'dim>,
    __marker: PhantomData<(Invariant<'dim>, Invariant<'scope>, Invariant<'range>)>,
}

impl<'head, 'tail, 'n> Partition<'head, 'tail, 'n> {
    /// Returns the midpoint of the partition.
    #[inline]
    pub const fn midpoint(&self) -> IdxInc<'n> {
        unsafe { IdxInc::new_unbound(self.head.unbound) }
    }

    /// Returns the midpoint of the partition.
    #[inline]
    pub const fn flip(&self) -> Partition<'tail, 'head, 'n> {
        Partition {
            head: self.tail,
            tail: self.head,
            __marker: PhantomData,
        }
    }
}

/// Lifetime branded length
/// # Safety
/// The type's safety invariant is that all instances of this type with the same lifetime
/// correspond to the same length.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Dim<'n> {
    unbound: usize,
    __marker: PhantomData<Invariant<'n>>,
}
impl PartialEq for Dim<'_> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        equator::debug_assert!(self.unbound == other.unbound);
        true
    }
}
impl Eq for Dim<'_> {}

impl PartialOrd for Dim<'_> {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        equator::debug_assert!(self.unbound == other.unbound);
        Some(core::cmp::Ordering::Equal)
    }
}
impl Ord for Dim<'_> {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        equator::debug_assert!(self.unbound == other.unbound);
        core::cmp::Ordering::Equal
    }
}

/// Lifetime branded index.
/// # Safety
/// The type's safety invariant is that all instances of this type are valid indices for
/// [`Dim<'n>`] and less than or equal to `I::Signed::MAX`.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Idx<'n, I: Index = usize> {
    unbound: I,
    __marker: PhantomData<Invariant<'n>>,
}

/// Lifetime branded partition index.
/// # Safety
/// The type's safety invariant is that all instances of this type are valid partition places
/// for [`Dim<'n>`] and less than or equal to `I::Signed::MAX`.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct IdxInc<'n, I: Index = usize> {
    unbound: I,
    __marker: PhantomData<Invariant<'n>>,
}

impl fmt::Debug for Dim<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.unbound.fmt(f)
    }
}
impl<I: Index> fmt::Debug for Idx<'_, I> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.unbound.fmt(f)
    }
}
impl<I: Index> fmt::Debug for IdxInc<'_, I> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.unbound.fmt(f)
    }
}
impl<I: Index> fmt::Debug for MaybeIdx<'_, I> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.unbound.to_signed() >= I::Signed::truncate(0) {
            self.unbound.fmt(f)
        } else {
            f.write_str("None")
        }
    }
}

impl<'n, I: Index> PartialEq<Dim<'n>> for Idx<'n, I> {
    #[inline(always)]
    fn eq(&self, other: &Dim<'n>) -> bool {
        equator::debug_assert!(self.unbound.zx() < other.unbound);

        false
    }
}
impl<'n, I: Index> PartialOrd<Dim<'n>> for Idx<'n, I> {
    #[inline(always)]
    fn partial_cmp(&self, other: &Dim<'n>) -> Option<core::cmp::Ordering> {
        equator::debug_assert!(self.unbound.zx() < other.unbound);

        Some(core::cmp::Ordering::Less)
    }
}

impl<'n, I: Index> PartialEq<Dim<'n>> for IdxInc<'n, I> {
    #[inline(always)]
    fn eq(&self, other: &Dim<'n>) -> bool {
        equator::debug_assert!(self.unbound.zx() <= other.unbound);

        self.unbound.zx() == other.unbound
    }
}

impl<'n, I: Index> PartialOrd<Dim<'n>> for IdxInc<'n, I> {
    #[inline(always)]
    fn partial_cmp(&self, other: &Dim<'n>) -> Option<core::cmp::Ordering> {
        equator::debug_assert!(self.unbound.zx() <= other.unbound);

        Some(if self.unbound.zx() == other.unbound {
            core::cmp::Ordering::Equal
        } else {
            core::cmp::Ordering::Less
        })
    }
}

impl Dim<'static> {
    pub const ONE: Self = Dim {
        unbound: 1,
        __marker: PhantomData,
    };
}

impl<'n> Dim<'n> {
    pub fn with<R>(dim: usize, f: impl for<'dim> FnOnce(Dim<'dim>) -> R) -> R {
        f(unsafe { Self::new_unbound(dim) })
    }

    /// Create new branded value with an arbitrary brand.
    /// # Safety
    /// See struct safety invariant.
    #[inline(always)]
    pub const unsafe fn new_unbound(dim: usize) -> Self {
        Self {
            unbound: dim,
            __marker: PhantomData,
        }
    }

    /// Create new branded value with a unique brand.
    #[inline(always)]
    pub fn new(dim: usize, guard: Guard<'n>) -> Self {
        _ = guard;
        Self {
            unbound: dim,
            __marker: PhantomData,
        }
    }

    /// Returns the unconstrained value.
    #[inline(always)]
    pub const fn unbound(self) -> usize {
        self.unbound
    }

    /// Partitions `self` into two segments as specifiedd by the midpoint.
    #[inline]
    pub const fn partition<'head, 'tail>(
        self,
        midpoint: IdxInc<'n>,
        head: Guard<'head>,
        tail: Guard<'tail>,
    ) -> Partition<'head, 'tail, 'n> {
        _ = (head, tail);
        unsafe {
            Partition {
                head: Dim::new_unbound(midpoint.unbound),
                tail: Dim::new_unbound(self.unbound - midpoint.unbound),
                __marker: PhantomData,
            }
        }
    }

    /// Partitions `self` into two segments.
    #[inline]
    #[track_caller]
    pub fn head_partition<'head, 'tail>(
        self,
        head: Dim<'head>,
        tail: Guard<'tail>,
    ) -> Partition<'head, 'tail, 'n> {
        _ = (head, tail);
        let midpoint = IdxInc::new_checked(head.unbound(), self);
        unsafe {
            Partition {
                head,
                tail: Dim::new_unbound(self.unbound - midpoint.unbound),
                __marker: PhantomData,
            }
        }
    }

    #[inline]
    pub fn advance(self, start: Idx<'n>, len: usize) -> IdxInc<'n> {
        let len = Ord::min(self.unbound.saturating_sub(start.unbound), len);
        IdxInc {
            unbound: start.unbound + len,
            __marker: PhantomData,
        }
    }

    #[inline]
    pub fn flip(self, i: IdxInc<'n>) -> IdxInc<'n> {
        IdxInc {
            unbound: self.unbound - i.unbound,
            __marker: PhantomData,
        }
    }

    /// Returns an iterator over the indices between `0` and `self`.
    #[inline]
    pub fn indices(self) -> impl Clone + ExactSizeIterator + DoubleEndedIterator<Item = Idx<'n>> {
        (0..self.unbound).map(|i| unsafe { Idx::new_unbound(i) })
    }

    /// Returns an iterator over the indices between `0` and `self`.
    #[inline]
    #[cfg(feature = "rayon")]
    pub fn par_indices(self) -> impl rayon::iter::IndexedParallelIterator<Item = Idx<'n>> {
        use rayon::prelude::*;
        (0..self.unbound)
            .into_par_iter()
            .map(|i| unsafe { Idx::new_unbound(i) })
    }

    #[inline(always)]
    pub fn full<'full, 'scope, T>(
        self,
        node: GhostNode<'scope, 'full, T>,
    ) -> (Segment<'scope, 'n, 'n>, T)
    where
        'n: 'full,
    {
        (
            Segment {
                start: zero(),
                end: IdxInc {
                    unbound: self.unbound,
                    __marker: PhantomData,
                },
                __marker: PhantomData,
            },
            node.child,
        )
    }

    #[inline(always)]
    pub fn split<'scope, 'range, Node, Seg: DimSplit<'scope, 'n, Node>>(
        self,
        segments: Seg,
        node: Node,
    ) -> (Seg::Output, Seg::Metadata) {
        Seg::in_dim(segments, &mut zero(), self, node)
    }
}

#[derive(Clone, Debug)]
pub struct SegmentIter<'scope, 'dim, 'range> {
    start: usize,
    end: usize,
    __marker: PhantomData<(Invariant<'scope>, Invariant<'dim>, Invariant<'range>)>,
}

impl<'scope, 'dim, 'range> Iterator for SegmentIter<'scope, 'dim, 'range> {
    type Item = SegmentIdx<'scope, 'dim, 'range>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            let idx = SegmentIdx {
                unbound: self.start,
                __marker: PhantomData,
            };
            self.start += 1;
            Some(idx)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.end - self.start;
        (len, Some(len))
    }
}

impl<'scope, 'dim, 'range> DoubleEndedIterator for SegmentIter<'scope, 'dim, 'range> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            self.end -= 1;
            let idx = SegmentIdx {
                unbound: self.end,
                __marker: PhantomData,
            };
            Some(idx)
        }
    }
}

impl<'scope, 'dim, 'range> ExactSizeIterator for SegmentIter<'scope, 'dim, 'range> {
    #[inline]
    fn len(&self) -> usize {
        self.end - self.start
    }
}

impl<'scope, 'dim, 'range> IntoIterator for Segment<'scope, 'dim, 'range> {
    type Item = SegmentIdx<'scope, 'dim, 'range>;
    type IntoIter = SegmentIter<'scope, 'dim, 'range>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        SegmentIter {
            start: self.start.unbound,
            end: self.end.unbound,
            __marker: PhantomData,
        }
    }
}

impl<'scope, 'dim, 'range> SegmentIdx<'scope, 'dim, 'range> {
    #[inline(always)]
    pub const unsafe fn new_unbound<'head, 'tail>(index: Idx<'dim>) -> Self {
        Self {
            unbound: index.unbound,
            __marker: PhantomData,
        }
    }

    #[inline]
    pub const fn local(self) -> Idx<'dim> {
        unsafe { Idx::new_unbound(self.unbound) }
    }
}

impl<'scope, 'dim, 'range> SegmentIdxInc<'scope, 'dim, 'range> {
    #[inline(always)]
    pub const unsafe fn new_unbound<'head, 'tail>(index: IdxInc<'dim>) -> Self {
        Self {
            unbound: index.unbound,
            __marker: PhantomData,
        }
    }

    #[inline]
    pub const fn local(self) -> IdxInc<'dim> {
        unsafe { IdxInc::new_unbound(self.unbound) }
    }
}

impl<'scope, 'dim, 'range> Segment<'scope, 'dim, 'range> {
    #[inline(always)]
    pub fn split<Node, Seg: SegmentSplit<'scope, 'dim, 'range, Node>>(
        self,
        segments: Seg,
        node: Node,
    ) -> (Seg::Output, Seg::Metadata) {
        Seg::in_segment(segments, &mut { self.start() }, self.end(), node)
    }

    #[inline(always)]
    pub const unsafe fn new_unbound<'head, 'tail>(start: IdxInc<'dim>, end: IdxInc<'dim>) -> Self {
        Self {
            start,
            end,
            __marker: PhantomData,
        }
    }
    #[inline]
    pub const fn start(self) -> SegmentIdxInc<'scope, 'dim, 'range> {
        unsafe { SegmentIdxInc::new_unbound(self.start) }
    }

    #[inline]
    pub const fn end(self) -> SegmentIdxInc<'scope, 'dim, 'range> {
        unsafe { SegmentIdxInc::new_unbound(self.end) }
    }

    #[inline]
    pub const fn len(self) -> Dim<'range> {
        unsafe { Dim::new_unbound(self.end.unbound - self.start.unbound) }
    }

    #[inline]
    #[track_caller]
    pub fn idx(self, value: usize) -> SegmentIdx<'scope, 'dim, 'range> {
        assert!(all(value >= *self.start, value < *self.end));
        SegmentIdx {
            unbound: value,
            __marker: PhantomData,
        }
    }

    #[inline]
    #[track_caller]
    pub fn idx_inc(self, value: usize) -> SegmentIdxInc<'scope, 'dim, 'range> {
        assert!(all(value >= *self.start, value <= *self.end));
        SegmentIdxInc {
            unbound: value,
            __marker: PhantomData,
        }
    }

    #[inline]
    pub fn try_idx(self, value: usize) -> Option<SegmentIdx<'scope, 'dim, 'range>> {
        if value >= *self.start && value < *self.end {
            Some(SegmentIdx {
                unbound: value,
                __marker: PhantomData,
            })
        } else {
            None
        }
    }

    #[inline]
    pub fn try_idx_inc(self, value: usize) -> Option<SegmentIdxInc<'scope, 'dim, 'range>> {
        if value >= *self.start && value <= *self.end {
            Some(SegmentIdxInc {
                unbound: value,
                __marker: PhantomData,
            })
        } else {
            None
        }
    }

    #[inline]
    pub fn local<I: ToLocal<'scope, 'dim, 'range>>(self, idx: I) -> I::Local {
        I::to_local(self, idx)
    }
    #[inline]
    pub fn global<I: ToGlobal<'scope, 'dim, 'range>>(self, idx: I) -> I::Global {
        I::to_global(self, idx)
    }

    #[inline]
    pub const fn from_local(self, idx: Idx<'range>) -> SegmentIdx<'scope, 'dim, 'range> {
        SegmentIdx {
            unbound: self.start.unbound + idx.unbound,
            __marker: PhantomData,
        }
    }

    #[inline]
    pub const fn from_global(self, idx: SegmentIdx<'scope, 'dim, 'range>) -> Idx<'range> {
        Idx {
            unbound: idx.unbound - self.start.unbound,
            __marker: PhantomData,
        }
    }

    #[inline]
    pub const fn from_local_inc(self, inc: IdxInc<'range>) -> SegmentIdxInc<'scope, 'dim, 'range> {
        SegmentIdxInc {
            unbound: self.start.unbound + inc.unbound,
            __marker: PhantomData,
        }
    }

    #[inline]
    pub const fn from_global_inc(self, inc: SegmentIdxInc<'scope, 'dim, 'range>) -> IdxInc<'range> {
        IdxInc {
            unbound: inc.unbound - self.start.unbound,
            __marker: PhantomData,
        }
    }
}

impl<'n, I: Index> Idx<'n, I> {
    /// Create new branded value with an arbitrary brand.
    /// # Safety
    /// See struct safety invariant.
    #[inline(always)]
    pub const unsafe fn new_unbound(idx: I) -> Self {
        Self {
            unbound: idx,
            __marker: PhantomData,
        }
    }

    /// Create new branded value with the same brand as `dim`.
    /// # Safety
    /// The behavior is undefined unless `idx < dim` and `idx <= I::Signed::MAX`.
    #[inline(always)]
    pub unsafe fn new_unchecked(idx: I, dim: Dim<'n>) -> Self {
        equator::debug_assert!(all(
            idx.zx() < dim.unbound,
            idx <= I::from_signed(I::Signed::MAX),
        ));

        Self {
            unbound: idx,
            __marker: PhantomData,
        }
    }

    /// Create new branded value with the same brand as `dim`.
    /// # Panics
    /// Panics unless `idx < dim` and `idx <= I::Signed::MAX`.
    #[inline(always)]
    #[track_caller]
    pub fn new_checked(idx: I, dim: Dim<'n>) -> Self {
        equator::assert!(all(
            idx.zx() < dim.unbound,
            idx <= I::from_signed(I::Signed::MAX),
        ));

        Self {
            unbound: idx,
            __marker: PhantomData,
        }
    }

    /// Returns the unconstrained value.
    #[inline(always)]
    pub const fn unbound(self) -> I {
        self.unbound
    }

    /// Zero-extends the internal value into a `usize`.
    #[inline(always)]
    pub fn zx(self) -> Idx<'n> {
        Idx {
            unbound: self.unbound.zx(),
            __marker: PhantomData,
        }
    }
}

impl<'n> IdxInc<'n> {
    pub const ZERO: Self = unsafe { Self::new_unbound(0) };
}

#[inline(always)]
pub const fn zero<'n>() -> IdxInc<'n> {
    IdxInc {
        unbound: 0,
        __marker: PhantomData,
    }
}

impl<'n, I: Index> IdxInc<'n, I> {
    /// Create new branded value with an arbitrary brand.
    /// # Safety
    /// See struct safety invariant.
    #[inline(always)]
    pub const unsafe fn new_unbound(idx: I) -> Self {
        Self {
            unbound: idx,
            __marker: PhantomData,
        }
    }

    /// Create new branded value with the same brand as `dim`.
    /// # Safety
    /// The behavior is undefined unless `idx <= dim`.
    #[inline(always)]
    pub unsafe fn new_unchecked(idx: I, dim: Dim<'n>) -> Self {
        equator::debug_assert!(all(
            idx.zx() <= dim.unbound,
            idx <= I::from_signed(I::Signed::MAX),
        ));

        Self {
            unbound: idx,
            __marker: PhantomData,
        }
    }

    /// Create new branded value with the same brand as `dim`.
    /// # Panics
    /// Panics unless `idx <= dim`.
    #[inline(always)]
    #[track_caller]
    pub fn new_checked(idx: I, dim: Dim<'n>) -> Self {
        equator::assert!(all(
            idx.zx() <= dim.unbound,
            idx <= I::from_signed(I::Signed::MAX),
        ));

        Self {
            unbound: idx,
            __marker: PhantomData,
        }
    }

    /// Returns the unconstrained value.
    #[inline(always)]
    pub const fn unbound(self) -> I {
        self.unbound
    }

    /// Zero-extends the internal value into a `usize`.
    #[inline(always)]
    pub fn zx(self) -> IdxInc<'n> {
        IdxInc {
            unbound: self.unbound.zx(),
            __marker: PhantomData,
        }
    }
}

impl<'n> IdxInc<'n> {
    /// Returns an iterator over the indices between `self` and `to`.
    #[inline]
    pub fn to(
        self,
        upper: IdxInc<'n>,
    ) -> impl Clone + ExactSizeIterator + DoubleEndedIterator<Item = Idx<'n>> {
        (self.unbound..upper.unbound).map(|i| unsafe { Idx::new_unbound(i) })
    }

    /// Returns an iterator over the indices between `self` and `to`.
    #[inline]
    pub fn range_to(
        self,
        upper: IdxInc<'n>,
    ) -> impl Clone + ExactSizeIterator + DoubleEndedIterator<Item = Idx<'n>> {
        (self.unbound..upper.unbound).map(|i| unsafe { Idx::new_unbound(i) })
    }
}

impl Unbind for Dim<'_> {
    #[inline(always)]
    unsafe fn new_unbound(idx: usize) -> Self {
        Self::new_unbound(idx)
    }

    #[inline(always)]
    fn unbound(self) -> usize {
        self.unbound
    }
}
impl<I: Index> Unbind<I> for Idx<'_, I> {
    #[inline(always)]
    unsafe fn new_unbound(idx: I) -> Self {
        Self::new_unbound(idx)
    }

    #[inline(always)]
    fn unbound(self) -> I {
        self.unbound
    }
}
impl<I: Index> Unbind<I> for IdxInc<'_, I> {
    #[inline(always)]
    unsafe fn new_unbound(idx: I) -> Self {
        Self::new_unbound(idx)
    }

    #[inline(always)]
    fn unbound(self) -> I {
        self.unbound
    }
}

impl<I: Index> Unbind<I::Signed> for MaybeIdx<'_, I> {
    #[inline(always)]
    unsafe fn new_unbound(idx: I::Signed) -> Self {
        Self::new_unbound(I::from_signed(idx))
    }

    #[inline(always)]
    fn unbound(self) -> I::Signed {
        self.unbound.to_signed()
    }
}

impl<'dim> ShapeIdx for Dim<'dim> {
    type Idx<I: Index> = Idx<'dim, I>;
    type IdxInc<I: Index> = IdxInc<'dim, I>;
    type MaybeIdx<I: Index> = MaybeIdx<'dim, I>;
}

impl<'dim> Shape for Dim<'dim> {}

impl<'n, I: Index> From<Idx<'n, I>> for IdxInc<'n, I> {
    #[inline(always)]
    fn from(value: Idx<'n, I>) -> Self {
        Self {
            unbound: value.unbound,
            __marker: PhantomData,
        }
    }
}
impl<'scope, 'dim, 'n> From<SegmentIdx<'scope, 'dim, 'n>> for SegmentIdxInc<'scope, 'dim, 'n> {
    #[inline(always)]
    fn from(value: SegmentIdx<'scope, 'dim, 'n>) -> Self {
        Self {
            unbound: value.unbound,
            __marker: PhantomData,
        }
    }
}

impl<'scope, 'dim, 'n> SegmentIdx<'scope, 'dim, 'n> {
    #[inline]
    pub fn next(self) -> SegmentIdxInc<'scope, 'dim, 'n> {
        SegmentIdxInc {
            unbound: self.unbound + 1,
            __marker: PhantomData,
        }
    }

    #[inline]
    pub fn to_incl(self) -> SegmentIdxInc<'scope, 'dim, 'n> {
        SegmentIdxInc {
            unbound: self.unbound,
            __marker: PhantomData,
        }
    }
}

impl<'n> From<Dim<'n>> for IdxInc<'n> {
    #[inline(always)]
    fn from(value: Dim<'n>) -> Self {
        Self {
            unbound: value.unbound,
            __marker: PhantomData,
        }
    }
}

impl<'n, I: Index> From<Idx<'n, I>> for MaybeIdx<'n, I> {
    #[inline(always)]
    fn from(value: Idx<'n, I>) -> Self {
        Self {
            unbound: value.unbound,
            __marker: PhantomData,
        }
    }
}

impl<'size> Dim<'size> {
    /// Check that the index is bounded by `self`, or panic otherwise.
    #[track_caller]
    #[inline]
    pub fn check<I: Index>(self, idx: I) -> Idx<'size, I> {
        Idx::new_checked(idx, self)
    }

    /// Check that the index is bounded by `self`, or panic otherwise.
    #[track_caller]
    #[inline]
    pub fn idx<I: Index>(self, idx: I) -> Idx<'size, I> {
        Idx::new_checked(idx, self)
    }

    /// Check that the index is bounded by `self`, or panic otherwise.
    #[track_caller]
    #[inline]
    pub fn idx_inc<I: Index>(self, idx: I) -> IdxInc<'size, I> {
        IdxInc::new_checked(idx, self)
    }

    /// Check that the index is bounded by `self`, or return `None` otherwise.
    #[inline]
    pub fn try_check<I: Index>(self, idx: I) -> Option<Idx<'size, I>> {
        if idx.zx() < self.unbound() {
            Some(unsafe { Idx::new_unbound(idx) })
        } else {
            None
        }
    }
}

impl<'n> Idx<'n> {
    /// Truncate `self` to a smaller type `I`.
    pub fn truncate<I: Index>(self) -> Idx<'n, I> {
        unsafe { Idx::new_unbound(I::truncate(self.unbound())) }
    }
}

impl<'n, I: Index> Idx<'n, I> {
    /// Returns the index, bounded inclusively by the value tied to `'n`.
    #[inline]
    pub const fn to_incl(self) -> IdxInc<'n, I> {
        unsafe { IdxInc::new_unbound(self.unbound()) }
    }
    /// Returns the next index, bounded inclusively by the value tied to `'n`.
    #[inline]
    pub fn next(self) -> IdxInc<'n, I> {
        unsafe { IdxInc::new_unbound(self.unbound() + I::truncate(1)) }
    }
    /// Returns the index, bounded inclusively by the value tied to `'n`.
    #[inline]
    pub fn excl(self) -> IdxInc<'n, I> {
        unsafe { IdxInc::new_unbound(self.unbound()) }
    }

    /// Assert that the values of `slice` are all bounded by `size`.
    #[track_caller]
    #[inline]
    pub fn from_slice_mut_checked<'a>(slice: &'a mut [I], size: Dim<'n>) -> &'a mut [Idx<'n, I>] {
        Self::from_slice_ref_checked(slice, size);
        unsafe { &mut *(slice as *mut _ as *mut _) }
    }

    /// Assume that the values of `slice` are all bounded by the value tied to `'n`.
    #[track_caller]
    #[inline]
    pub unsafe fn from_slice_mut_unchecked<'a>(slice: &'a mut [I]) -> &'a mut [Idx<'n, I>] {
        unsafe { &mut *(slice as *mut _ as *mut _) }
    }

    /// Assert that the values of `slice` are all bounded by `size`.
    #[track_caller]
    pub fn from_slice_ref_checked<'a>(slice: &'a [I], size: Dim<'n>) -> &'a [Idx<'n, I>] {
        for &idx in slice {
            Self::new_checked(idx, size);
        }
        unsafe { &*(slice as *const _ as *const _) }
    }

    /// Assume that the values of `slice` are all bounded by the value tied to `'n`.
    #[track_caller]
    #[inline]
    pub unsafe fn from_slice_ref_unchecked<'a>(slice: &'a [I]) -> &'a [Idx<'n, I>] {
        unsafe { &*(slice as *const _ as *const _) }
    }
}

/// `I` value smaller than the size corresponding to the lifetime `'n`, or `None`.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct MaybeIdx<'n, I: Index = usize> {
    unbound: I,
    __marker: PhantomData<Invariant<'n>>,
}

impl<'n, I: Index> MaybeIdx<'n, I> {
    /// Returns an index value.
    #[inline]
    pub fn from_index(idx: Idx<'n, I>) -> Self {
        unsafe { Self::new_unbound(idx.unbound()) }
    }
    /// Returns a `None` value.
    #[inline]
    pub fn none() -> Self {
        unsafe { Self::new_unbound(I::truncate(usize::MAX)) }
    }

    /// Returns a constrained index value if `idx` is nonnegative, `None` otherwise.
    #[inline]
    #[track_caller]
    pub fn new_checked(idx: I::Signed, size: Dim<'n>) -> Self {
        assert!((idx.sx() as isize) < size.unbound() as isize);
        Self {
            unbound: I::from_signed(idx),
            __marker: PhantomData,
        }
    }

    /// Returns a constrained index value if `idx` is nonnegative, `None` otherwise.
    #[inline]
    pub unsafe fn new_unchecked(idx: I::Signed, size: Dim<'n>) -> Self {
        debug_assert!((idx.sx() as isize) < size.unbound() as isize);
        Self {
            unbound: I::from_signed(idx),
            __marker: PhantomData,
        }
    }

    /// Returns a constrained index value if `idx` is nonnegative, `None` otherwise.
    #[inline]
    pub unsafe fn new_unbound(idx: I) -> Self {
        Self {
            unbound: idx,
            __marker: PhantomData,
        }
    }

    /// Returns the inner value.
    #[inline]
    pub fn unbound(self) -> I {
        self.unbound
    }

    /// Returns the index if available, or `None` otherwise.
    #[inline]
    pub fn idx(self) -> Option<Idx<'n, I>> {
        if self.unbound.to_signed() >= I::Signed::truncate(0) {
            Some(unsafe { Idx::new_unbound(self.unbound()) })
        } else {
            None
        }
    }

    /// Sign extend the value.
    #[inline]
    pub fn sx(self) -> MaybeIdx<'n> {
        unsafe { MaybeIdx::new_unbound(self.unbound.to_signed().sx()) }
    }

    /// Assert that the values of `slice` are all bounded by `size`.
    #[track_caller]
    #[inline]
    pub fn from_slice_mut_checked<'a>(
        slice: &'a mut [I::Signed],
        size: Dim<'n>,
    ) -> &'a mut [MaybeIdx<'n, I>] {
        Self::from_slice_ref_checked(slice, size);
        unsafe { &mut *(slice as *mut _ as *mut _) }
    }

    /// Assume that the values of `slice` are all bounded by the value tied to `'n`.
    #[track_caller]
    #[inline]
    pub unsafe fn from_slice_mut_unchecked<'a>(
        slice: &'a mut [I::Signed],
    ) -> &'a mut [MaybeIdx<'n, I>] {
        unsafe { &mut *(slice as *mut _ as *mut _) }
    }

    /// Assert that the values of `slice` are all bounded by `size`.
    #[track_caller]
    pub fn from_slice_ref_checked<'a>(
        slice: &'a [I::Signed],
        size: Dim<'n>,
    ) -> &'a [MaybeIdx<'n, I>] {
        for &idx in slice {
            Self::new_checked(idx, size);
        }
        unsafe { &*(slice as *const _ as *const _) }
    }

    /// Convert a constrained slice to an unconstrained one.
    #[track_caller]
    pub fn as_slice_ref<'a>(slice: &'a [MaybeIdx<'n, I>]) -> &'a [I::Signed] {
        unsafe { &*(slice as *const _ as *const _) }
    }

    /// Assume that the values of `slice` are all bounded by the value tied to `'n`.
    #[track_caller]
    #[inline]
    pub unsafe fn from_slice_ref_unchecked<'a>(slice: &'a [I::Signed]) -> &'a [MaybeIdx<'n, I>] {
        unsafe { &*(slice as *const _ as *const _) }
    }
}

impl core::ops::Deref for Dim<'_> {
    type Target = usize;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.unbound
    }
}
impl<I: Index> core::ops::Deref for MaybeIdx<'_, I> {
    type Target = I::Signed;
    #[inline]
    fn deref(&self) -> &Self::Target {
        bytemuck::cast_ref(&self.unbound)
    }
}
impl<I: Index> core::ops::Deref for Idx<'_, I> {
    type Target = I;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.unbound
    }
}
impl<I: Index> core::ops::Deref for IdxInc<'_, I> {
    type Target = I;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.unbound
    }
}

impl core::ops::Deref for SegmentIdx<'_, '_, '_> {
    type Target = usize;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.unbound
    }
}

impl core::ops::Deref for SegmentIdxInc<'_, '_, '_> {
    type Target = usize;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.unbound
    }
}

/// Array of length equal to the value tied to `'n`.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Array<'n, T> {
    __marker: PhantomData<Invariant<'n>>,
    unbound: [T],
}

impl<'n, T> Array<'n, T> {
    /// Returns a constrained array after checking that its length matches `size`.
    #[inline]
    #[track_caller]
    pub fn from_ref<'a>(slice: &'a [T], size: Dim<'n>) -> &'a Self {
        assert!(slice.len() == size.unbound());
        unsafe { &*(slice as *const [T] as *const Self) }
    }

    /// Returns a constrained array after checking that its length matches `size`.
    #[inline]
    #[track_caller]
    pub fn from_mut<'a>(slice: &'a mut [T], size: Dim<'n>) -> &'a mut Self {
        assert!(slice.len() == size.unbound());
        unsafe { &mut *(slice as *mut [T] as *mut Self) }
    }

    /// Returns the unconstrained slice.
    #[inline]
    #[track_caller]
    pub fn as_ref(&self) -> &[T] {
        unsafe { &*(self as *const _ as *const _) }
    }

    /// Returns the unconstrained slice.
    #[inline]
    #[track_caller]
    pub fn as_mut<'a>(&mut self) -> &'a mut [T] {
        unsafe { &mut *(self as *mut _ as *mut _) }
    }

    /// Returns the length of `self`.
    #[inline]
    pub fn len(&self) -> Dim<'n> {
        unsafe { Dim::new_unbound(self.unbound.len()) }
    }

    #[inline]
    pub fn segments<'a, 'scope, S: ArraySplit<'scope, 'n, 'a>>(
        &'a self,
        segments: S,
    ) -> S::ArrayRefSegments<T> {
        S::array_ref_segments(segments, self)
    }

    #[inline]
    pub fn segment<'scope, 'HEAD>(&self, first: Segment<'scope, 'n, 'HEAD>) -> &Array<'HEAD, T> {
        let ptr = self.as_ref().as_ptr();
        unsafe {
            &*(core::slice::from_raw_parts(ptr.add(first.start.unbound), first.len().unbound)
                as *const _ as *const Array<'HEAD, T>)
        }
    }

    #[inline]
    pub fn segments_mut<'a, 'scope, S: ArraySplit<'scope, 'n, 'a>>(
        &'a mut self,
        segments: S,
        disjoint: S::Disjoint,
    ) -> S::ArrayMutSegments<T> {
        S::array_mut_segments(segments, self, disjoint)
    }

    #[inline]
    pub fn segment_mut<'scope, 'HEAD>(
        &mut self,
        first: Segment<'scope, 'n, 'HEAD>,
    ) -> &mut Array<'HEAD, T> {
        let ptr = self.as_mut().as_mut_ptr();
        unsafe {
            &mut *(core::slice::from_raw_parts_mut(
                ptr.add(first.start.unbound),
                first.len().unbound,
            ) as *mut _ as *mut Array<'HEAD, T>)
        }
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for Array<'_, T> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.unbound.fmt(f)
    }
}

impl<'n, T> core::ops::Index<Range<IdxInc<'n>>> for Array<'n, T> {
    type Output = [T];
    #[track_caller]
    fn index(&self, idx: Range<IdxInc<'n>>) -> &Self::Output {
        #[cfg(debug_assertions)]
        {
            &self.unbound[idx.start.unbound()..idx.end.unbound()]
        }
        #[cfg(not(debug_assertions))]
        unsafe {
            self.unbound
                .get_unchecked(idx.start.unbound()..idx.end.unbound())
        }
    }
}
impl<'n, T> core::ops::IndexMut<Range<IdxInc<'n>>> for Array<'n, T> {
    #[track_caller]
    fn index_mut(&mut self, idx: Range<IdxInc<'n>>) -> &mut Self::Output {
        #[cfg(debug_assertions)]
        {
            &mut self.unbound[idx.start.unbound()..idx.end.unbound()]
        }
        #[cfg(not(debug_assertions))]
        unsafe {
            self.unbound
                .get_unchecked_mut(idx.start.unbound()..idx.end.unbound())
        }
    }
}
impl<'n, T> core::ops::Index<Idx<'n>> for Array<'n, T> {
    type Output = T;
    #[track_caller]
    fn index(&self, idx: Idx<'n>) -> &Self::Output {
        #[cfg(debug_assertions)]
        {
            &self.unbound[idx.unbound()]
        }
        #[cfg(not(debug_assertions))]
        unsafe {
            self.unbound.get_unchecked(idx.unbound())
        }
    }
}
impl<'n, T> core::ops::IndexMut<Idx<'n>> for Array<'n, T> {
    #[track_caller]
    fn index_mut(&mut self, idx: Idx<'n>) -> &mut Self::Output {
        #[cfg(debug_assertions)]
        {
            &mut self.unbound[idx.unbound()]
        }
        #[cfg(not(debug_assertions))]
        unsafe {
            self.unbound.get_unchecked_mut(idx.unbound())
        }
    }
}
