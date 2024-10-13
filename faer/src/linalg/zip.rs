use std::marker::PhantomData;

use crate::{
    col::{ColGeneric, ColMutGeneric, ColRefGeneric},
    mat::{MatGeneric, MatMutGeneric, MatRefGeneric},
    row::{RowGeneric, RowMutGeneric, RowRefGeneric},
    ContiguousFwd, Idx, Shape, Stride, Unbind,
};
use equator::{assert, debug_assert};
use faer_traits::{help, Container, Unit};
use reborrow::*;

pub trait IntoView {
    type Target;

    fn into_view(self) -> Self::Target;
}

impl<'a, C: Container, T, Rows: Shape, Cols: Shape> IntoView
    for &'a mut MatGeneric<C, T, Rows, Cols>
{
    type Target = MatMutGeneric<'a, C, T, Rows, Cols, ContiguousFwd>;
    #[inline]
    fn into_view(self) -> Self::Target {
        self.as_mut().try_as_col_major_mut().unwrap()
    }
}
impl<'a, C: Container, T, Rows: Shape, Cols: Shape> IntoView for &'a MatGeneric<C, T, Rows, Cols> {
    type Target = MatRefGeneric<'a, C, T, Rows, Cols, ContiguousFwd>;
    #[inline]
    fn into_view(self) -> Self::Target {
        self.as_ref().try_as_col_major().unwrap()
    }
}

impl<'a, C: Container, T, Len: Shape> IntoView for &'a mut ColGeneric<C, T, Len> {
    type Target = ColMutGeneric<'a, C, T, Len, ContiguousFwd>;
    #[inline]
    fn into_view(self) -> Self::Target {
        self.as_mut().try_as_col_major_mut().unwrap()
    }
}
impl<'a, C: Container, T, Len: Shape> IntoView for &'a ColGeneric<C, T, Len> {
    type Target = ColRefGeneric<'a, C, T, Len, ContiguousFwd>;
    #[inline]
    fn into_view(self) -> Self::Target {
        self.as_ref().try_as_col_major().unwrap()
    }
}

impl<'a, C: Container, T, Len: Shape> IntoView for &'a mut RowGeneric<C, T, Len> {
    type Target = RowMutGeneric<'a, C, T, Len, ContiguousFwd>;
    #[inline]
    fn into_view(self) -> Self::Target {
        self.as_mut().try_as_row_major_mut().unwrap()
    }
}
impl<'a, C: Container, T, Len: Shape> IntoView for &'a RowGeneric<C, T, Len> {
    type Target = RowRefGeneric<'a, C, T, Len, ContiguousFwd>;
    #[inline]
    fn into_view(self) -> Self::Target {
        self.as_ref().try_as_row_major().unwrap()
    }
}

impl<'a, C: Container, T, Rows: Shape, Cols: Shape, RStride: Stride, CStride: Stride> IntoView
    for MatMutGeneric<'a, C, T, Rows, Cols, RStride, CStride>
{
    type Target = Self;
    #[inline]
    fn into_view(self) -> Self::Target {
        self
    }
}
impl<'a, C: Container, T, Rows: Shape, Cols: Shape, RStride: Stride, CStride: Stride> IntoView
    for MatRefGeneric<'a, C, T, Rows, Cols, RStride, CStride>
{
    type Target = Self;
    #[inline]
    fn into_view(self) -> Self::Target {
        self
    }
}

impl<'a, C: Container, T, Rows: Shape, Cols: Shape, RStride: Stride, CStride: Stride> IntoView
    for &'a MatMutGeneric<'_, C, T, Rows, Cols, RStride, CStride>
{
    type Target = MatRefGeneric<'a, C, T, Rows, Cols, RStride, CStride>;
    #[inline]
    fn into_view(self) -> Self::Target {
        self.rb()
    }
}
impl<'a, C: Container, T, Rows: Shape, Cols: Shape, RStride: Stride, CStride: Stride> IntoView
    for &'a MatRefGeneric<'_, C, T, Rows, Cols, RStride, CStride>
{
    type Target = MatRefGeneric<'a, C, T, Rows, Cols, RStride, CStride>;
    #[inline]
    fn into_view(self) -> Self::Target {
        *self
    }
}

impl<'a, C: Container, T, Rows: Shape, Cols: Shape, RStride: Stride, CStride: Stride> IntoView
    for &'a mut MatMutGeneric<'_, C, T, Rows, Cols, RStride, CStride>
{
    type Target = MatMutGeneric<'a, C, T, Rows, Cols, RStride, CStride>;
    #[inline]
    fn into_view(self) -> Self::Target {
        self.rb_mut()
    }
}
impl<'a, C: Container, T, Rows: Shape, Cols: Shape, RStride: Stride, CStride: Stride> IntoView
    for &'a mut MatRefGeneric<'_, C, T, Rows, Cols, RStride, CStride>
{
    type Target = MatRefGeneric<'a, C, T, Rows, Cols, RStride, CStride>;
    #[inline]
    fn into_view(self) -> Self::Target {
        *self
    }
}

impl<'a, C: Container, T, Rows: Shape, RStride: Stride> IntoView
    for ColMutGeneric<'a, C, T, Rows, RStride>
{
    type Target = Self;
    #[inline]
    fn into_view(self) -> Self::Target {
        self
    }
}
impl<'a, C: Container, T, Rows: Shape, RStride: Stride> IntoView
    for ColRefGeneric<'a, C, T, Rows, RStride>
{
    type Target = Self;
    #[inline]
    fn into_view(self) -> Self::Target {
        self
    }
}

impl<'a, C: Container, T, Rows: Shape, RStride: Stride> IntoView
    for &'a ColMutGeneric<'_, C, T, Rows, RStride>
{
    type Target = ColRefGeneric<'a, C, T, Rows, RStride>;
    #[inline]
    fn into_view(self) -> Self::Target {
        self.rb()
    }
}
impl<'a, C: Container, T, Rows: Shape, RStride: Stride> IntoView
    for &'a ColRefGeneric<'_, C, T, Rows, RStride>
{
    type Target = ColRefGeneric<'a, C, T, Rows, RStride>;
    #[inline]
    fn into_view(self) -> Self::Target {
        *self
    }
}

impl<'a, C: Container, T, Rows: Shape, RStride: Stride> IntoView
    for &'a mut ColMutGeneric<'_, C, T, Rows, RStride>
{
    type Target = ColMutGeneric<'a, C, T, Rows, RStride>;
    #[inline]
    fn into_view(self) -> Self::Target {
        self.rb_mut()
    }
}
impl<'a, C: Container, T, Rows: Shape, RStride: Stride> IntoView
    for &'a mut ColRefGeneric<'_, C, T, Rows, RStride>
{
    type Target = ColRefGeneric<'a, C, T, Rows, RStride>;
    #[inline]
    fn into_view(self) -> Self::Target {
        *self
    }
}

impl<'a, C: Container, T, Cols: Shape, CStride: Stride> IntoView
    for RowMutGeneric<'a, C, T, Cols, CStride>
{
    type Target = Self;
    #[inline]
    fn into_view(self) -> Self::Target {
        self
    }
}
impl<'a, C: Container, T, Cols: Shape, CStride: Stride> IntoView
    for RowRefGeneric<'a, C, T, Cols, CStride>
{
    type Target = Self;
    #[inline]
    fn into_view(self) -> Self::Target {
        self
    }
}

impl<'a, C: Container, T, Cols: Shape, CStride: Stride> IntoView
    for &'a RowMutGeneric<'_, C, T, Cols, CStride>
{
    type Target = RowRefGeneric<'a, C, T, Cols, CStride>;
    #[inline]
    fn into_view(self) -> Self::Target {
        self.rb()
    }
}
impl<'a, C: Container, T, Cols: Shape, CStride: Stride> IntoView
    for &'a RowRefGeneric<'_, C, T, Cols, CStride>
{
    type Target = RowRefGeneric<'a, C, T, Cols, CStride>;
    #[inline]
    fn into_view(self) -> Self::Target {
        *self
    }
}

impl<'a, C: Container, T, Cols: Shape, CStride: Stride> IntoView
    for &'a mut RowMutGeneric<'_, C, T, Cols, CStride>
{
    type Target = RowMutGeneric<'a, C, T, Cols, CStride>;
    #[inline]
    fn into_view(self) -> Self::Target {
        self.rb_mut()
    }
}
impl<'a, C: Container, T, Cols: Shape, CStride: Stride> IntoView
    for &'a mut RowRefGeneric<'_, C, T, Cols, CStride>
{
    type Target = RowRefGeneric<'a, C, T, Cols, CStride>;
    #[inline]
    fn into_view(self) -> Self::Target {
        *self
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Diag {
    Skip,
    Include,
}

/// Matrix layout transformation. Used for zipping optimizations.
#[derive(Copy, Clone)]
pub enum MatLayoutTransform {
    /// Matrix is used as-is.
    None,
    /// Matrix rows are reversed.
    ReverseRows,
    /// Matrix is transposed.
    Transpose,
    /// Matrix is transposed, then rows are reversed.
    TransposeReverseRows,
}

/// Vector layout transformation. Used for zipping optimizations.
#[derive(Copy, Clone)]
pub enum VecLayoutTransform {
    /// Vector is used as-is.
    None,
    /// Vector is reversed.
    Reverse,
}

/// Type with a given matrix shape.
pub trait MatIndex {
    /// Type of rows.
    type Rows: Copy + Eq + core::fmt::Debug;
    /// Type of columns.
    type Cols: Copy + Eq + core::fmt::Debug;
    /// Returns the number of rows.
    fn nrows(this: &Self) -> Self::Rows;
    /// Returns the number of columns.
    fn ncols(this: &Self) -> Self::Cols;

    /// Indexing type.
    type Index: Copy;
    /// Layout transformation type.
    type LayoutTransform: Copy;

    /// Item produced by the zipped views.
    type Item;

    /// Matrix type with type erased dimensions.
    type Dyn: MatIndex<
        Dyn = Self::Dyn,
        LayoutTransform = Self::LayoutTransform,
        Item = Self::Item,
        Slice = Self::Slice,
    >;

    type Slice: for<'a> SliceFamily<'a, Self::Item>;

    /// Returns slice at index of length `n_elems`.
    unsafe fn get_slice_unchecked<'a>(
        this: &'a mut Self,
        idx: Self::Index,
        n_elems: usize,
    ) -> <Self::Slice as SliceFamily<'a, Self::Item>>::Slice;

    /// Converts a type erased index back to its original representation.
    unsafe fn from_dyn_idx(idx: <Self::Dyn as MatIndex>::Index) -> Self::Index;

    /// Get the item at the given index, skipping bound checks.
    unsafe fn get_unchecked(this: &mut Self, index: Self::Index) -> Self::Item;
    /// Get the item at the given slice position, skipping bound checks.
    unsafe fn next_unchecked<'a>(
        slice: &mut <Self::Slice as SliceFamily<'a, Self::Item>>::Slice,
    ) -> Self::Item;

    /// Checks if the zipped matrices are contiguous.
    fn is_contiguous(this: &Self) -> bool;
    /// Computes the preferred iteration layout of the matrices.
    fn preferred_layout(this: &Self) -> Self::LayoutTransform;
    /// Applies the layout transformation to the matrices.
    fn with_layout(this: Self, layout: Self::LayoutTransform) -> Self::Dyn;
}

pub trait SliceFamily<'a, T, Outlives = &'a Self> {
    type Slice;
}
pub struct Slice<C: Container, T>(C::Of<T>);
pub struct SliceRef<'b, C: Container, T>(C::Of<&'b T>);
pub struct SliceMut<'b, C: Container, T>(C::Of<&'b mut T>);

impl<'a, C: Container, T> SliceFamily<'a, C::Of<T>> for Slice<C, T> {
    type Slice = C::Of<&'a [T]>;
}
impl<'a, 'b, C: Container, T> SliceFamily<'a, C::Of<&'b T>> for SliceRef<'b, C, T> {
    type Slice = C::Of<&'b [T]>;
}
impl<'a, 'b, C: Container, T> SliceFamily<'a, C::Of<&'b mut T>> for SliceMut<'b, C, T> {
    type Slice = C::Of<&'b mut [T]>;
}
impl<'a, T, F: SliceFamily<'a, T>> SliceFamily<'a, Last<T>> for Last<F> {
    type Slice = Last<F::Slice>;
}
impl<'a, T, U, F: SliceFamily<'a, T>, G: SliceFamily<'a, U>> SliceFamily<'a, Zip<T, U>>
    for Zip<F, G>
{
    type Slice = Zip<F::Slice, G::Slice>;
}

/// Single matrix.
#[derive(Copy, Clone, Debug)]
pub struct LastEq<Rows, Cols, Mat>(pub Mat, pub PhantomData<(Rows, Cols)>);

/// Single element.
#[derive(Copy, Clone, Debug)]
pub struct Last<Mat>(pub Mat);

/// Zipped matrices.
#[derive(Copy, Clone, Debug)]
pub struct ZipEq<Rows, Cols, Head, Tail>(pub Head, pub Tail, PhantomData<(Rows, Cols)>);

/// Zipped elements.
#[derive(Copy, Clone, Debug)]
pub struct Zip<Head, Tail>(pub Head, pub Tail);

/// Single matrix view.
impl<
        Rows: Copy + Eq + core::fmt::Debug,
        Cols: Copy + Eq + core::fmt::Debug,
        Head: MatIndex<Rows = Rows, Cols = Cols>,
        Tail: MatIndex<Rows = Rows, Cols = Cols>,
    > ZipEq<Rows, Cols, Head, Tail>
{
    /// Creates a zipped matrix, after asserting that the dimensions match.
    #[inline(always)]
    #[track_caller]
    pub fn new(head: Head, tail: Tail) -> Self {
        assert!(all(
            Head::nrows(&head) == Tail::nrows(&tail),
            Head::ncols(&head) == Tail::ncols(&tail),
        ));
        Self(head, tail, PhantomData)
    }

    /// Creates a zipped matrix, assuming that the dimensions match.
    #[inline(always)]
    #[track_caller]
    pub fn new_unchecked(head: Head, tail: Tail) -> Self {
        debug_assert!(all(
            Head::nrows(&head) == Tail::nrows(&tail),
            Head::ncols(&head) == Tail::ncols(&tail),
        ));
        Self(head, tail, PhantomData)
    }
}

impl<
        Rows: Copy + Eq + core::fmt::Debug,
        Cols: Copy + Eq + core::fmt::Debug,
        Mat: MatIndex<Rows = Rows, Cols = Cols>,
    > MatIndex for LastEq<Rows, Cols, Mat>
{
    type Rows = Mat::Rows;
    type Cols = Mat::Cols;
    #[inline(always)]
    fn nrows(this: &Self) -> Self::Rows {
        Mat::nrows(&this.0)
    }
    #[inline(always)]
    fn ncols(this: &Self) -> Self::Cols {
        Mat::ncols(&this.0)
    }
    type Index = Mat::Index;
    type LayoutTransform = Mat::LayoutTransform;

    type Item = Last<Mat::Item>;
    type Dyn = LastEq<<Mat::Dyn as MatIndex>::Rows, <Mat::Dyn as MatIndex>::Cols, Mat::Dyn>;
    type Slice = Last<Mat::Slice>;

    #[inline]
    unsafe fn get_slice_unchecked<'a>(
        this: &'a mut Self,
        idx: Self::Index,
        n_elems: usize,
    ) -> <Self::Slice as SliceFamily<'a, Self::Item>>::Slice {
        Last(Mat::get_slice_unchecked(&mut this.0, idx, n_elems))
    }
    #[inline]
    unsafe fn from_dyn_idx(idx: <Self::Dyn as MatIndex>::Index) -> Self::Index {
        Mat::from_dyn_idx(idx)
    }
    #[inline]
    unsafe fn get_unchecked(this: &mut Self, index: Self::Index) -> Self::Item {
        Last(Mat::get_unchecked(&mut this.0, index))
    }
    #[inline]
    unsafe fn next_unchecked<'a>(
        slice: &mut <Self::Slice as SliceFamily<'a, Self::Item>>::Slice,
    ) -> Self::Item {
        Last(Mat::next_unchecked(&mut slice.0))
    }
    #[inline]
    fn is_contiguous(this: &Self) -> bool {
        Mat::is_contiguous(&this.0)
    }
    #[inline]
    fn preferred_layout(this: &Self) -> Self::LayoutTransform {
        Mat::preferred_layout(&this.0)
    }
    #[inline]
    fn with_layout(this: Self, layout: Self::LayoutTransform) -> Self::Dyn {
        LastEq(Mat::with_layout(this.0, layout), PhantomData)
    }
}

impl<
        Rows: Copy + Eq + core::fmt::Debug,
        Cols: Copy + Eq + core::fmt::Debug,
        L: MatIndex<Rows = Rows, Cols = Cols>,
        R: MatIndex<
            Rows = Rows,
            Cols = Cols,
            Dyn: MatIndex<
                Rows = <L::Dyn as MatIndex>::Rows,
                Cols = <L::Dyn as MatIndex>::Cols,
                Index = <L::Dyn as MatIndex>::Index,
            >,
            Index = L::Index,
            LayoutTransform = L::LayoutTransform,
        >,
    > MatIndex for ZipEq<Rows, Cols, L, R>
{
    type Rows = L::Rows;
    type Cols = L::Cols;
    #[inline(always)]
    fn nrows(this: &Self) -> Self::Rows {
        L::nrows(&this.0)
    }
    #[inline(always)]
    fn ncols(this: &Self) -> Self::Cols {
        L::ncols(&this.0)
    }

    type Index = L::Index;
    type LayoutTransform = L::LayoutTransform;

    type Item = Zip<L::Item, R::Item>;
    type Dyn = ZipEq<<L::Dyn as MatIndex>::Rows, <L::Dyn as MatIndex>::Cols, L::Dyn, R::Dyn>;
    type Slice = Zip<L::Slice, R::Slice>;

    #[inline]
    unsafe fn get_slice_unchecked<'a>(
        this: &'a mut Self,
        idx: Self::Index,
        n_elems: usize,
    ) -> <Self::Slice as SliceFamily<'a, Self::Item>>::Slice {
        Zip(
            L::get_slice_unchecked(&mut this.0, idx, n_elems),
            R::get_slice_unchecked(&mut this.1, idx, n_elems),
        )
    }
    #[inline]
    unsafe fn from_dyn_idx(idx: <Self::Dyn as MatIndex>::Index) -> Self::Index {
        L::from_dyn_idx(idx)
    }
    #[inline]
    unsafe fn get_unchecked(this: &mut Self, index: Self::Index) -> Self::Item {
        Zip(
            L::get_unchecked(&mut this.0, index),
            R::get_unchecked(&mut this.1, index),
        )
    }
    #[inline]
    unsafe fn next_unchecked<'a>(
        slice: &mut <Self::Slice as SliceFamily<'a, Self::Item>>::Slice,
    ) -> Self::Item {
        Zip(
            L::next_unchecked(&mut slice.0),
            R::next_unchecked(&mut slice.1),
        )
    }
    #[inline]
    fn is_contiguous(this: &Self) -> bool {
        L::is_contiguous(&this.0) && R::is_contiguous(&this.1)
    }
    #[inline]
    fn preferred_layout(this: &Self) -> Self::LayoutTransform {
        L::preferred_layout(&this.0)
    }
    #[inline]
    fn with_layout(this: Self, layout: Self::LayoutTransform) -> Self::Dyn {
        ZipEq(
            L::with_layout(this.0, layout),
            R::with_layout(this.1, layout),
            PhantomData,
        )
    }
}

impl<'b, C: Container, T, Rows: Shape, Cols: Shape, RStride: Stride, CStride: Stride> MatIndex
    for MatMutGeneric<'b, C, T, Rows, Cols, RStride, CStride>
{
    type Rows = Rows;
    type Cols = Cols;
    type Index = (Idx<Rows>, Idx<Cols>);
    type LayoutTransform = MatLayoutTransform;

    #[inline]
    fn nrows(this: &Self) -> Self::Rows {
        this.nrows()
    }
    #[inline]
    fn ncols(this: &Self) -> Self::Cols {
        this.ncols()
    }

    type Item = C::Of<&'b mut T>;
    type Dyn = MatMutGeneric<'b, C, T, usize, usize, isize, isize>;
    type Slice = SliceMut<'b, C, T>;

    #[inline]
    unsafe fn get_slice_unchecked<'a>(
        this: &'a mut Self,
        idx: Self::Index,
        n_elems: usize,
    ) -> <Self::Slice as SliceFamily<'a, Self::Item>>::Slice {
        help!(C);
        map!(
            this.ptr_inbounds_at_mut(idx.0, idx.1),
            ptr,
            core::slice::from_raw_parts_mut(ptr, n_elems)
        )
    }

    #[inline]
    unsafe fn from_dyn_idx(idx: <Self::Dyn as MatIndex>::Index) -> Self::Index {
        (
            Idx::<Rows>::new_unbound(idx.0),
            Idx::<Cols>::new_unbound(idx.1),
        )
    }
    #[inline]
    unsafe fn get_unchecked(this: &mut Self, (i, j): Self::Index) -> Self::Item {
        help!(C);
        map!(this.rb().ptr_inbounds_at_mut(i, j), ptr, &mut *ptr)
    }

    #[inline(always)]
    unsafe fn next_unchecked<'a>(
        slice: &mut <Self::Slice as SliceFamily<'a, Self::Item>>::Slice,
    ) -> Self::Item {
        help!(C);
        let (head, tail) = unzip!(map!(
            as_mut!(*slice),
            slice,
            core::mem::take(slice).split_first_mut().unwrap_unchecked()
        ));
        *slice = tail;
        head
    }

    #[inline(always)]
    fn is_contiguous(this: &Self) -> bool {
        this.row_stride().element_stride() == 1
    }
    #[inline(always)]
    fn preferred_layout(this: &Self) -> Self::LayoutTransform {
        let rs = this.row_stride().element_stride();
        let cs = this.col_stride().element_stride();
        let nrows = this.nrows().unbound();
        let ncols = this.ncols().unbound();

        if nrows > 1 && rs == 1 {
            MatLayoutTransform::None
        } else if nrows > 1 && rs == -1 {
            MatLayoutTransform::ReverseRows
        } else if ncols > 1 && cs == 1 {
            MatLayoutTransform::Transpose
        } else if ncols > 1 && cs == -1 {
            MatLayoutTransform::TransposeReverseRows
        } else {
            MatLayoutTransform::None
        }
    }
    #[inline(always)]
    fn with_layout(this: Self, layout: Self::LayoutTransform) -> Self::Dyn {
        use MatLayoutTransform::*;
        let this = this.as_dyn_mut().as_dyn_stride_mut();
        match layout {
            None => this,
            ReverseRows => this.reverse_rows_mut(),
            Transpose => this.transpose_mut(),
            TransposeReverseRows => this.transpose_mut().reverse_rows_mut(),
        }
    }
}

impl<'b, C: Container, T, Rows: Shape, Cols: Shape, RStride: Stride, CStride: Stride> MatIndex
    for MatRefGeneric<'b, C, T, Rows, Cols, RStride, CStride>
{
    type Rows = Rows;
    type Cols = Cols;
    type Index = (Idx<Rows>, Idx<Cols>);
    type LayoutTransform = MatLayoutTransform;

    #[inline]
    fn nrows(this: &Self) -> Self::Rows {
        this.nrows()
    }
    #[inline]
    fn ncols(this: &Self) -> Self::Cols {
        this.ncols()
    }

    type Item = C::Of<&'b T>;
    type Dyn = MatRefGeneric<'b, C, T, usize, usize, isize, isize>;
    type Slice = SliceRef<'b, C, T>;

    #[inline]
    unsafe fn get_slice_unchecked<'a>(
        this: &'a mut Self,
        idx: Self::Index,
        n_elems: usize,
    ) -> <Self::Slice as SliceFamily<'a, Self::Item>>::Slice {
        help!(C);
        map!(
            this.ptr_inbounds_at(idx.0, idx.1),
            ptr,
            core::slice::from_raw_parts(ptr, n_elems)
        )
    }

    #[inline]
    unsafe fn from_dyn_idx(idx: <Self::Dyn as MatIndex>::Index) -> Self::Index {
        (
            Idx::<Rows>::new_unbound(idx.0),
            Idx::<Cols>::new_unbound(idx.1),
        )
    }
    #[inline]
    unsafe fn get_unchecked(this: &mut Self, (i, j): Self::Index) -> Self::Item {
        help!(C);
        map!(this.rb().ptr_inbounds_at(i, j), ptr, &*ptr)
    }

    #[inline(always)]
    unsafe fn next_unchecked<'a>(
        slice: &mut <Self::Slice as SliceFamily<'a, Self::Item>>::Slice,
    ) -> Self::Item {
        help!(C);
        let (head, tail) = unzip!(map!(
            as_mut!(*slice),
            slice,
            core::mem::take(slice).split_first().unwrap_unchecked()
        ));
        *slice = tail;
        head
    }

    #[inline(always)]
    fn is_contiguous(this: &Self) -> bool {
        this.row_stride().element_stride() == 1
    }
    #[inline(always)]
    fn preferred_layout(this: &Self) -> Self::LayoutTransform {
        let rs = this.row_stride().element_stride();
        let cs = this.col_stride().element_stride();
        let nrows = this.nrows().unbound();
        let ncols = this.ncols().unbound();

        if nrows > 1 && rs == 1 {
            MatLayoutTransform::None
        } else if nrows > 1 && rs == -1 {
            MatLayoutTransform::ReverseRows
        } else if ncols > 1 && cs == 1 {
            MatLayoutTransform::Transpose
        } else if ncols > 1 && cs == -1 {
            MatLayoutTransform::TransposeReverseRows
        } else {
            MatLayoutTransform::None
        }
    }
    #[inline(always)]
    fn with_layout(this: Self, layout: Self::LayoutTransform) -> Self::Dyn {
        use MatLayoutTransform::*;
        let this = this.as_dyn().as_dyn_stride();
        match layout {
            None => this,
            ReverseRows => this.reverse_rows(),
            Transpose => this.transpose(),
            TransposeReverseRows => this.transpose().reverse_rows(),
        }
    }
}

impl<'b, C: Container, T, Len: Shape, Strd: Stride> MatIndex
    for ColMutGeneric<'b, C, T, Len, Strd>
{
    type Rows = Len;
    type Cols = Unit;
    type Index = Idx<Len>;
    type LayoutTransform = VecLayoutTransform;

    #[inline]
    fn nrows(this: &Self) -> Self::Rows {
        this.nrows()
    }
    #[inline]
    fn ncols(_: &Self) -> Self::Cols {
        Unit
    }

    type Item = C::Of<&'b mut T>;
    type Dyn = ColMutGeneric<'b, C, T, usize, isize>;
    type Slice = SliceMut<'b, C, T>;

    #[inline]
    unsafe fn get_slice_unchecked<'a>(
        this: &'a mut Self,
        idx: Self::Index,
        n_elems: usize,
    ) -> <Self::Slice as SliceFamily<'a, Self::Item>>::Slice {
        help!(C);
        map!(
            this.ptr_inbounds_at_mut(idx),
            ptr,
            core::slice::from_raw_parts_mut(ptr, n_elems)
        )
    }

    #[inline]
    unsafe fn from_dyn_idx(idx: <Self::Dyn as MatIndex>::Index) -> Self::Index {
        Idx::<Len>::new_unbound(idx)
    }
    #[inline]
    unsafe fn get_unchecked(this: &mut Self, i: Self::Index) -> Self::Item {
        help!(C);
        map!(this.rb().ptr_inbounds_at_mut(i), ptr, &mut *ptr)
    }

    #[inline(always)]
    unsafe fn next_unchecked<'a>(
        slice: &mut <Self::Slice as SliceFamily<'a, Self::Item>>::Slice,
    ) -> Self::Item {
        help!(C);
        let (head, tail) = unzip!(map!(
            as_mut!(*slice),
            slice,
            core::mem::take(slice).split_first_mut().unwrap_unchecked()
        ));
        *slice = tail;
        head
    }

    #[inline(always)]
    fn is_contiguous(this: &Self) -> bool {
        this.row_stride().element_stride() == 1
    }
    #[inline(always)]
    fn preferred_layout(this: &Self) -> Self::LayoutTransform {
        let strd = this.row_stride().element_stride();
        let len = this.nrows().unbound();

        if len > 1 && strd == 1 {
            VecLayoutTransform::None
        } else if len > 1 && strd == -1 {
            VecLayoutTransform::Reverse
        } else {
            VecLayoutTransform::None
        }
    }
    #[inline(always)]
    fn with_layout(this: Self, layout: Self::LayoutTransform) -> Self::Dyn {
        use VecLayoutTransform::*;
        let this = this.as_dyn_rows_mut().as_dyn_stride_mut();
        match layout {
            None => this,
            Reverse => this.reverse_rows_mut(),
        }
    }
}

impl<'b, C: Container, T, Len: Shape, Strd: Stride> MatIndex
    for RowMutGeneric<'b, C, T, Len, Strd>
{
    type Rows = Unit;
    type Cols = Len;
    type Index = Idx<Len>;
    type LayoutTransform = VecLayoutTransform;

    #[inline]
    fn nrows(_: &Self) -> Self::Rows {
        Unit
    }
    #[inline]
    fn ncols(this: &Self) -> Self::Cols {
        this.ncols()
    }

    type Item = C::Of<&'b mut T>;
    type Dyn = RowMutGeneric<'b, C, T, usize, isize>;
    type Slice = SliceMut<'b, C, T>;

    #[inline]
    unsafe fn get_slice_unchecked<'a>(
        this: &'a mut Self,
        idx: Self::Index,
        n_elems: usize,
    ) -> <Self::Slice as SliceFamily<'a, Self::Item>>::Slice {
        help!(C);
        map!(
            this.ptr_inbounds_at_mut(idx),
            ptr,
            core::slice::from_raw_parts_mut(ptr, n_elems)
        )
    }

    #[inline]
    unsafe fn from_dyn_idx(idx: <Self::Dyn as MatIndex>::Index) -> Self::Index {
        Idx::<Len>::new_unbound(idx)
    }
    #[inline]
    unsafe fn get_unchecked(this: &mut Self, i: Self::Index) -> Self::Item {
        help!(C);
        map!(this.rb().ptr_inbounds_at_mut(i), ptr, &mut *ptr)
    }

    #[inline(always)]
    unsafe fn next_unchecked<'a>(
        slice: &mut <Self::Slice as SliceFamily<'a, Self::Item>>::Slice,
    ) -> Self::Item {
        help!(C);
        let (head, tail) = unzip!(map!(
            as_mut!(*slice),
            slice,
            core::mem::take(slice).split_first_mut().unwrap_unchecked()
        ));
        *slice = tail;
        head
    }

    #[inline(always)]
    fn is_contiguous(this: &Self) -> bool {
        this.col_stride().element_stride() == 1
    }
    #[inline(always)]
    fn preferred_layout(this: &Self) -> Self::LayoutTransform {
        let strd = this.col_stride().element_stride();
        let len = this.ncols().unbound();

        if len > 1 && strd == 1 {
            VecLayoutTransform::None
        } else if len > 1 && strd == -1 {
            VecLayoutTransform::Reverse
        } else {
            VecLayoutTransform::None
        }
    }
    #[inline(always)]
    fn with_layout(this: Self, layout: Self::LayoutTransform) -> Self::Dyn {
        use VecLayoutTransform::*;
        let this = this.as_dyn_cols_mut().as_dyn_stride_mut();
        match layout {
            None => this,
            Reverse => this.reverse_cols_mut(),
        }
    }
}

impl<'b, C: Container, T, Len: Shape, Strd: Stride> MatIndex
    for ColRefGeneric<'b, C, T, Len, Strd>
{
    type Rows = Len;
    type Cols = Unit;
    type Index = Idx<Len>;
    type LayoutTransform = VecLayoutTransform;

    #[inline]
    fn nrows(this: &Self) -> Self::Rows {
        this.nrows()
    }
    #[inline]
    fn ncols(_: &Self) -> Self::Cols {
        Unit
    }

    type Item = C::Of<&'b T>;
    type Dyn = ColRefGeneric<'b, C, T, usize, isize>;
    type Slice = SliceRef<'b, C, T>;

    #[inline]
    unsafe fn get_slice_unchecked<'a>(
        this: &'a mut Self,
        idx: Self::Index,
        n_elems: usize,
    ) -> <Self::Slice as SliceFamily<'a, Self::Item>>::Slice {
        help!(C);
        map!(
            this.ptr_inbounds_at(idx),
            ptr,
            core::slice::from_raw_parts(ptr, n_elems)
        )
    }

    #[inline]
    unsafe fn from_dyn_idx(idx: <Self::Dyn as MatIndex>::Index) -> Self::Index {
        Idx::<Len>::new_unbound(idx)
    }
    #[inline]
    unsafe fn get_unchecked(this: &mut Self, i: Self::Index) -> Self::Item {
        help!(C);
        map!(this.rb().ptr_inbounds_at(i), ptr, &*ptr)
    }

    #[inline(always)]
    unsafe fn next_unchecked<'a>(
        slice: &mut <Self::Slice as SliceFamily<'a, Self::Item>>::Slice,
    ) -> Self::Item {
        help!(C);
        let (head, tail) = unzip!(map!(
            as_mut!(*slice),
            slice,
            core::mem::take(slice).split_first().unwrap_unchecked()
        ));
        *slice = tail;
        head
    }

    #[inline(always)]
    fn is_contiguous(this: &Self) -> bool {
        this.row_stride().element_stride() == 1
    }
    #[inline(always)]
    fn preferred_layout(this: &Self) -> Self::LayoutTransform {
        let strd = this.row_stride().element_stride();
        let len = this.nrows().unbound();

        if len > 1 && strd == 1 {
            VecLayoutTransform::None
        } else if len > 1 && strd == -1 {
            VecLayoutTransform::Reverse
        } else {
            VecLayoutTransform::None
        }
    }
    #[inline(always)]
    fn with_layout(this: Self, layout: Self::LayoutTransform) -> Self::Dyn {
        use VecLayoutTransform::*;
        let this = this.as_dyn_rows().as_dyn_stride();
        match layout {
            None => this,
            Reverse => this.reverse_rows(),
        }
    }
}

impl<'b, C: Container, T, Len: Shape, Strd: Stride> MatIndex
    for RowRefGeneric<'b, C, T, Len, Strd>
{
    type Rows = Unit;
    type Cols = Len;
    type Index = Idx<Len>;
    type LayoutTransform = VecLayoutTransform;

    #[inline]
    fn nrows(_: &Self) -> Self::Rows {
        Unit
    }
    #[inline]
    fn ncols(this: &Self) -> Self::Cols {
        this.ncols()
    }

    type Item = C::Of<&'b T>;
    type Dyn = RowRefGeneric<'b, C, T, usize, isize>;
    type Slice = SliceRef<'b, C, T>;

    #[inline]
    unsafe fn get_slice_unchecked<'a>(
        this: &'a mut Self,
        idx: Self::Index,
        n_elems: usize,
    ) -> <Self::Slice as SliceFamily<'a, Self::Item>>::Slice {
        help!(C);
        map!(
            this.ptr_inbounds_at(idx),
            ptr,
            core::slice::from_raw_parts(ptr, n_elems)
        )
    }

    #[inline]
    unsafe fn from_dyn_idx(idx: <Self::Dyn as MatIndex>::Index) -> Self::Index {
        Idx::<Len>::new_unbound(idx)
    }
    #[inline]
    unsafe fn get_unchecked(this: &mut Self, i: Self::Index) -> Self::Item {
        help!(C);
        map!(this.rb().ptr_inbounds_at(i), ptr, &*ptr)
    }

    #[inline(always)]
    unsafe fn next_unchecked<'a>(
        slice: &mut <Self::Slice as SliceFamily<'a, Self::Item>>::Slice,
    ) -> Self::Item {
        help!(C);
        let (head, tail) = unzip!(map!(
            as_mut!(*slice),
            slice,
            core::mem::take(slice).split_first().unwrap_unchecked()
        ));
        *slice = tail;
        head
    }

    #[inline(always)]
    fn is_contiguous(this: &Self) -> bool {
        this.col_stride().element_stride() == 1
    }
    #[inline(always)]
    fn preferred_layout(this: &Self) -> Self::LayoutTransform {
        let strd = this.col_stride().element_stride();
        let len = this.ncols().unbound();

        if len > 1 && strd == 1 {
            VecLayoutTransform::None
        } else if len > 1 && strd == -1 {
            VecLayoutTransform::Reverse
        } else {
            VecLayoutTransform::None
        }
    }
    #[inline(always)]
    fn with_layout(this: Self, layout: Self::LayoutTransform) -> Self::Dyn {
        use VecLayoutTransform::*;
        let this = this.as_dyn_cols().as_dyn_stride();
        match layout {
            None => this,
            Reverse => this.reverse_cols(),
        }
    }
}

#[inline(always)]
fn annotate_noalias_mat<Z: MatIndex>(
    f: &mut impl FnMut(<Z as MatIndex>::Item),
    mut slice: <Z::Slice as SliceFamily<'_, Z::Item>>::Slice,
    i_begin: usize,
    i_end: usize,
    _j: usize,
) {
    for _ in i_begin..i_end {
        unsafe { f(Z::next_unchecked(&mut slice)) };
    }
}

#[inline(always)]
fn annotate_noalias_mat_with_index<
    Z: MatIndex<Index = (RowIdx, ColIdx), Dyn: MatIndex<Index = (usize, usize)>>,
    RowIdx,
    ColIdx,
>(
    f: &mut impl FnMut(RowIdx, ColIdx, <Z as MatIndex>::Item),
    mut slice: <Z::Slice as SliceFamily<'_, Z::Item>>::Slice,
    i_begin: usize,
    i_end: usize,
    j: usize,
    transpose: bool,
    reverse_rows: bool,
) {
    if !transpose {
        if !reverse_rows {
            for i in i_begin..i_end {
                unsafe {
                    let (ii, jj) = Z::from_dyn_idx((i, j));
                    f(ii, jj, Z::next_unchecked(&mut slice))
                };
            }
        } else {
            for i in i_begin..i_end {
                unsafe {
                    let (ii, jj) = Z::from_dyn_idx((i_begin + (i_end - i - 1), j));
                    f(ii, jj, Z::next_unchecked(&mut slice))
                };
            }
        }
    } else {
        if !reverse_rows {
            for i in i_begin..i_end {
                unsafe {
                    let (ii, jj) = Z::from_dyn_idx((j, i));
                    f(ii, jj, Z::next_unchecked(&mut slice))
                };
            }
        } else {
            for i in i_begin..i_end {
                unsafe {
                    let (ii, jj) = Z::from_dyn_idx((j, i_begin + (i_end - i - 1)));
                    f(ii, jj, Z::next_unchecked(&mut slice))
                };
            }
        }
    }
}

#[inline(always)]
fn annotate_noalias_col<Z: MatIndex>(
    f: &mut impl FnMut(<Z as MatIndex>::Item),
    mut slice: <Z::Slice as SliceFamily<'_, Z::Item>>::Slice,
    i_begin: usize,
    i_end: usize,
) {
    for _ in i_begin..i_end {
        unsafe { f(Z::next_unchecked(&mut slice)) };
    }
}

#[inline(always)]
fn annotate_noalias_col_with_index<
    Z: MatIndex<Index = Idx, Dyn: MatIndex<Item = Z::Item, Index = usize>>,
    Idx,
>(
    f: &mut impl FnMut(Idx, <Z as MatIndex>::Item),
    mut slice: <Z::Slice as SliceFamily<'_, Z::Item>>::Slice,
    i_begin: usize,
    i_end: usize,
    reverse: bool,
) {
    if !reverse {
        for i in i_begin..i_end {
            unsafe {
                let ii = Z::from_dyn_idx(i);
                f(ii, Z::next_unchecked(&mut slice))
            };
        }
    } else {
        for i in i_begin..i_end {
            unsafe {
                let ii = Z::from_dyn_idx(i_begin + (i_end - i - 1));
                f(ii, Z::next_unchecked(&mut slice))
            };
        }
    }
}

#[inline(always)]
fn for_each_mat<
    Z: MatIndex<
        Dyn: MatIndex<
            Item = Z::Item,
            Slice = Z::Slice,
            Rows = usize,
            Cols = usize,
            Index = (usize, usize),
        >,
    >,
>(
    z: Z,
    mut f: impl FnMut(<Z as MatIndex>::Item),
) {
    let layout = Z::preferred_layout(&z);
    let mut z = Z::with_layout(z, layout);

    let m = Z::Dyn::nrows(&z);
    let n = Z::Dyn::ncols(&z);
    if m == 0 || n == 0 {
        return;
    }

    unsafe {
        if Z::Dyn::is_contiguous(&z) {
            for j in 0..n {
                annotate_noalias_mat::<Z::Dyn>(
                    &mut f,
                    Z::Dyn::get_slice_unchecked(&mut z, (0, j), m),
                    0,
                    m,
                    j,
                );
            }
        } else {
            for j in 0..n {
                for i in 0..m {
                    f(Z::Dyn::get_unchecked(&mut z, (i, j)))
                }
            }
        }
    }
}

// TODO:
// - for_each_vec_with_index

#[inline(always)]
fn for_each_mat_with_index<
    RowIdx,
    ColIdx,
    Z: MatIndex<
        Index = (RowIdx, ColIdx),
        Dyn: MatIndex<
            Rows = usize,
            Cols = usize,
            Index = (usize, usize),
            Slice = Z::Slice,
            Item = Z::Item,
        >,
        LayoutTransform = MatLayoutTransform,
    >,
>(
    z: Z,
    mut f: impl FnMut(RowIdx, ColIdx, <Z as MatIndex>::Item),
) {
    let layout = Z::preferred_layout(&z);
    let mut z = Z::with_layout(z, layout);

    let m = Z::Dyn::nrows(&z);
    let n = Z::Dyn::ncols(&z);
    if m == 0 || n == 0 {
        return;
    }

    match layout {
        MatLayoutTransform::None => unsafe {
            if Z::Dyn::is_contiguous(&z) {
                for j in 0..n {
                    annotate_noalias_mat_with_index::<Z, _, _>(
                        &mut f,
                        Z::Dyn::get_slice_unchecked(&mut z, (0, j), m),
                        0,
                        m,
                        j,
                        false,
                        false,
                    );
                }
            } else {
                for j in 0..n {
                    for i in 0..m {
                        let (ii, jj) = Z::from_dyn_idx((i, j));
                        f(ii, jj, Z::Dyn::get_unchecked(&mut z, (i, j)))
                    }
                }
            }
        },
        MatLayoutTransform::ReverseRows => unsafe {
            if Z::Dyn::is_contiguous(&z) {
                for j in 0..n {
                    annotate_noalias_mat_with_index::<Z, _, _>(
                        &mut f,
                        Z::Dyn::get_slice_unchecked(&mut z, (0, j), m),
                        0,
                        m,
                        j,
                        false,
                        true,
                    );
                }
            } else {
                for j in 0..n {
                    for i in 0..m {
                        let (ii, jj) = Z::from_dyn_idx((m - i - 1, j));
                        f(ii, jj, Z::Dyn::get_unchecked(&mut z, (i, j)))
                    }
                }
            }
        },
        MatLayoutTransform::Transpose => unsafe {
            if Z::Dyn::is_contiguous(&z) {
                for j in 0..n {
                    annotate_noalias_mat_with_index::<Z, _, _>(
                        &mut f,
                        Z::Dyn::get_slice_unchecked(&mut z, (0, j), m),
                        0,
                        m,
                        j,
                        true,
                        false,
                    );
                }
            } else {
                for j in 0..n {
                    for i in 0..m {
                        let (ii, jj) = Z::from_dyn_idx((j, i));
                        f(ii, jj, Z::Dyn::get_unchecked(&mut z, (i, j)))
                    }
                }
            }
        },
        MatLayoutTransform::TransposeReverseRows => unsafe {
            if Z::Dyn::is_contiguous(&z) {
                for j in 0..n {
                    annotate_noalias_mat_with_index::<Z, _, _>(
                        &mut f,
                        Z::Dyn::get_slice_unchecked(&mut z, (0, j), m),
                        0,
                        m,
                        j,
                        true,
                        true,
                    );
                }
            } else {
                for j in 0..n {
                    for i in 0..m {
                        let (ii, jj) = Z::from_dyn_idx((j, m - i - 1));
                        f(ii, jj, Z::Dyn::get_unchecked(&mut z, (i, j)))
                    }
                }
            }
        },
    }
}

#[inline(always)]
fn for_each_mat_triangular_lower_with_index<
    RowIdx,
    ColIdx,
    Z: MatIndex<
        Index = (RowIdx, ColIdx),
        Dyn: MatIndex<
            Rows = usize,
            Cols = usize,
            Index = (usize, usize),
            Item = Z::Item,
            Slice = Z::Slice,
        >,
        LayoutTransform = MatLayoutTransform,
    >,
>(
    z: Z,
    diag: Diag,
    mut f: impl FnMut(RowIdx, ColIdx, <Z as MatIndex>::Item),
) {
    let layout = Z::preferred_layout(&z);
    let mut z = Z::with_layout(z, layout);

    let m = Z::Dyn::nrows(&z);
    let n = Z::Dyn::ncols(&z);
    let strict = match diag {
        Diag::Skip => true,
        Diag::Include => false,
    };
    let strict = strict as usize;

    if m == 0 || n == 0 {
        return;
    }

    match layout {
        MatLayoutTransform::None => unsafe {
            if Z::Dyn::is_contiguous(&z) {
                for j in 0..n {
                    let start = j + strict;
                    let end = m;
                    if start == end {
                        continue;
                    }
                    annotate_noalias_mat_with_index::<Z, _, _>(
                        &mut f,
                        Z::Dyn::get_slice_unchecked(&mut z, (start, j), end - start),
                        start,
                        end,
                        j,
                        false,
                        false,
                    );
                }
            } else {
                for j in 0..n {
                    let start = j + strict;
                    let end = m;
                    if start == end {
                        continue;
                    }
                    for i in start..end {
                        let (ii, jj) = Z::from_dyn_idx((i, j));
                        f(ii, jj, Z::Dyn::get_unchecked(&mut z, (i, j)))
                    }
                }
            }
        },
        MatLayoutTransform::ReverseRows => unsafe {
            if Z::Dyn::is_contiguous(&z) {
                for j in 0..Ord::min(m, n) {
                    let start = 0;
                    let end = m - j - strict;
                    if start == end {
                        continue;
                    }
                    annotate_noalias_mat_with_index::<Z, _, _>(
                        &mut f,
                        Z::Dyn::get_slice_unchecked(&mut z, (start, j), end - start),
                        j + strict + start,
                        j + strict + end,
                        j,
                        false,
                        true,
                    );
                }
            } else {
                for j in 0..Ord::min(m, n) {
                    let start = 0;
                    let end = m - j - strict;
                    if start == end {
                        continue;
                    }
                    for i in start..end {
                        let (ii, jj) = Z::from_dyn_idx((m - i - 1, j));
                        f(ii, jj, Z::Dyn::get_unchecked(&mut z, (i, j)))
                    }
                }
            }
        },
        MatLayoutTransform::Transpose => unsafe {
            if Z::Dyn::is_contiguous(&z) {
                for j in 0..n {
                    let start = 0;
                    let end = Ord::min(m, j + (1 - strict));
                    if start == end {
                        continue;
                    }
                    annotate_noalias_mat_with_index::<Z, _, _>(
                        &mut f,
                        Z::Dyn::get_slice_unchecked(&mut z, (0, j), end - start),
                        start,
                        end,
                        j,
                        true,
                        false,
                    );
                }
            } else {
                for j in 0..n {
                    let start = 0;
                    let end = Ord::min(m, j + (1 - strict));
                    if start == end {
                        continue;
                    }
                    for i in start..end {
                        let (ii, jj) = Z::from_dyn_idx((j, i));
                        f(ii, jj, Z::Dyn::get_unchecked(&mut z, (i, j)))
                    }
                }
            }
        },
        MatLayoutTransform::TransposeReverseRows => unsafe {
            if Z::Dyn::is_contiguous(&z) {
                for j in 0..n {
                    let start = m - Ord::min(j + (1 - strict) as usize, m);
                    let end = m;
                    if start == end {
                        continue;
                    }
                    annotate_noalias_mat_with_index::<Z, _, _>(
                        &mut f,
                        Z::Dyn::get_slice_unchecked(&mut z, (start, j), end - start),
                        0,
                        end - start,
                        j,
                        true,
                        true,
                    );
                }
            } else {
                for j in 0..n {
                    let start = m - Ord::min(j + (1 - strict) as usize, m);
                    let end = m;
                    if start == end {
                        continue;
                    }
                    for i in start..end {
                        let (ii, jj) = Z::from_dyn_idx((j, m - i - 1));
                        f(ii, jj, Z::Dyn::get_unchecked(&mut z, (i, j)))
                    }
                }
            }
        },
    }
}

#[inline(always)]
fn for_each_mat_triangular_upper_with_index<
    RowIdx,
    ColIdx,
    Z: MatIndex<
        Index = (RowIdx, ColIdx),
        Dyn: MatIndex<
            Rows = usize,
            Cols = usize,
            Index = (usize, usize),
            Item = Z::Item,
            Slice = Z::Slice,
        >,
        LayoutTransform = MatLayoutTransform,
    >,
>(
    z: Z,
    diag: Diag,
    mut f: impl FnMut(RowIdx, ColIdx, <Z as MatIndex>::Item),
) {
    let layout = Z::preferred_layout(&z);
    let mut z = Z::with_layout(z, layout);

    let m = Z::Dyn::nrows(&z);
    let n = Z::Dyn::ncols(&z);
    let strict = match diag {
        Diag::Skip => true,
        Diag::Include => false,
    };
    let strict = strict as usize;

    if m == 0 || n == 0 {
        return;
    }

    match layout {
        MatLayoutTransform::None => unsafe {
            if Z::Dyn::is_contiguous(&z) {
                for j in 0..n {
                    let start = 0;
                    let end = Ord::min(m, j + (1 - strict));
                    if start == end {
                        continue;
                    }

                    annotate_noalias_mat_with_index::<Z, _, _>(
                        &mut f,
                        Z::Dyn::get_slice_unchecked(&mut z, (start, j), end - start),
                        start,
                        end,
                        j,
                        false,
                        false,
                    );
                }
            } else {
                for j in 0..n {
                    let start = 0;
                    let end = Ord::min(m, j + (1 - strict));
                    if start == end {
                        continue;
                    }
                    for i in start..end {
                        let (ii, jj) = Z::from_dyn_idx((i, j));
                        f(ii, jj, Z::Dyn::get_unchecked(&mut z, (i, j)))
                    }
                }
            }
        },
        MatLayoutTransform::ReverseRows => unsafe {
            if Z::Dyn::is_contiguous(&z) {
                for j in 0..Ord::min(m, n) {
                    let start = m - Ord::min(j + (1 - strict) as usize, m);
                    let end = m;
                    if start == end {
                        continue;
                    }
                    annotate_noalias_mat_with_index::<Z, _, _>(
                        &mut f,
                        Z::Dyn::get_slice_unchecked(&mut z, (start, j), end - start),
                        0,
                        end - start,
                        j,
                        false,
                        true,
                    );
                }
            } else {
                for j in 0..Ord::min(m, n) {
                    let start = m - Ord::min(j + (1 - strict) as usize, m);
                    let end = m;
                    if start == end {
                        continue;
                    }
                    for i in start..end {
                        let (ii, jj) = Z::from_dyn_idx((m - i - 1, j));
                        f(ii, jj, Z::Dyn::get_unchecked(&mut z, (i, j)))
                    }
                }
            }
        },
        MatLayoutTransform::Transpose => unsafe {
            if Z::Dyn::is_contiguous(&z) {
                for j in 0..n {
                    let start = j + strict;
                    let end = m;
                    if start == end {
                        continue;
                    }
                    annotate_noalias_mat_with_index::<Z, _, _>(
                        &mut f,
                        Z::Dyn::get_slice_unchecked(&mut z, (start, j), end - start),
                        start,
                        end,
                        j,
                        true,
                        false,
                    );
                }
            } else {
                for j in 0..n {
                    let start = j + strict;
                    let end = m;
                    if start == end {
                        continue;
                    }
                    for i in start..end {
                        let (ii, jj) = Z::from_dyn_idx((j, i));
                        f(ii, jj, Z::Dyn::get_unchecked(&mut z, (i, j)))
                    }
                }
            }
        },
        MatLayoutTransform::TransposeReverseRows => unsafe {
            if Z::Dyn::is_contiguous(&z) {
                for j in 0..n {
                    let start = 0;
                    let end = m - j - strict;
                    if start == end {
                        continue;
                    }
                    annotate_noalias_mat_with_index::<Z, _, _>(
                        &mut f,
                        Z::Dyn::get_slice_unchecked(&mut z, (start, j), end - start),
                        j + strict,
                        j + strict + end - start,
                        j,
                        true,
                        true,
                    );
                }
            } else {
                for j in 0..n {
                    let start = 0;
                    let end = m - j - strict;
                    if start == end {
                        continue;
                    }
                    for i in start..end {
                        let (ii, jj) = Z::from_dyn_idx((j, m - i - 1));
                        f(ii, jj, Z::Dyn::get_unchecked(&mut z, (i, j)))
                    }
                }
            }
        },
    }
}

#[inline(always)]
fn for_each_mat_triangular_lower<
    Z: MatIndex<
        LayoutTransform = MatLayoutTransform,
        Dyn: MatIndex<
            LayoutTransform = MatLayoutTransform,
            Item = Z::Item,
            Slice = Z::Slice,
            Rows = usize,
            Cols = usize,
            Index = (usize, usize),
            Dyn = Z::Dyn,
        >,
    >,
>(
    z: Z,
    diag: Diag,
    transpose: bool,
    mut f: impl FnMut(<Z as MatIndex>::Item),
) {
    use MatLayoutTransform::*;

    let z = if transpose {
        Z::with_layout(z, MatLayoutTransform::Transpose)
    } else {
        Z::with_layout(z, MatLayoutTransform::None)
    };
    let layout = Z::Dyn::preferred_layout(&z);
    let mut z = Z::Dyn::with_layout(z, layout);

    let m = Z::Dyn::nrows(&z);
    let n = Z::Dyn::ncols(&z);
    let n = match layout {
        None | ReverseRows => Ord::min(m, n),
        Transpose | TransposeReverseRows => n,
    };
    if m == 0 || n == 0 {
        return;
    }

    let strict = match diag {
        Diag::Skip => true,
        Diag::Include => false,
    };

    unsafe {
        if Z::Dyn::is_contiguous(&z) {
            for j in 0..n {
                let (start, end) = match layout {
                    None => (j + strict as usize, m),
                    ReverseRows => (0, (m - (j + strict as usize))),
                    Transpose => (0, (j + !strict as usize).min(m)),
                    TransposeReverseRows => (m - ((j + !strict as usize).min(m)), m),
                };

                let len = end - start;
                if start == end {
                    continue;
                }

                annotate_noalias_mat::<Z::Dyn>(
                    &mut f,
                    Z::Dyn::get_slice_unchecked(&mut z, (start, j), len),
                    start,
                    end,
                    j,
                );
            }
        } else {
            for j in 0..n {
                let (start, end) = match layout {
                    None => (j + strict as usize, m),
                    ReverseRows => (0, (m - (j + strict as usize))),
                    Transpose => (0, (j + !strict as usize).min(m)),
                    TransposeReverseRows => (m - ((j + !strict as usize).min(m)), m),
                };
                if start == end {
                    continue;
                }

                for i in start..end {
                    f(Z::Dyn::get_unchecked(&mut z, (i, j)))
                }
            }
        }
    }
}

#[inline(always)]
fn for_each_col<
    Z: MatIndex<
        Dyn: MatIndex<Rows = usize, Cols = Unit, Index = usize, Item = Z::Item, Slice = Z::Slice>,
    >,
>(
    z: Z,
    mut f: impl FnMut(<Z as MatIndex>::Item),
) {
    let layout = Z::preferred_layout(&z);
    let mut z = Z::with_layout(z, layout);

    let m = Z::Dyn::nrows(&z);
    if m == 0 {
        return;
    }

    unsafe {
        if Z::Dyn::is_contiguous(&z) {
            annotate_noalias_col::<Z::Dyn>(&mut f, Z::Dyn::get_slice_unchecked(&mut z, 0, m), 0, m);
        } else {
            for i in 0..m {
                f(Z::Dyn::get_unchecked(&mut z, i))
            }
        }
    }
}

#[inline(always)]
fn for_each_col_with_index<
    Idx,
    Z: MatIndex<
        LayoutTransform = VecLayoutTransform,
        Index = Idx,
        Dyn: MatIndex<Rows = usize, Cols = Unit, Index = usize, Item = Z::Item, Slice = Z::Slice>,
    >,
>(
    z: Z,
    mut f: impl FnMut(Idx, <Z as MatIndex>::Item),
) {
    let layout = Z::preferred_layout(&z);
    let mut z = Z::with_layout(z, layout);

    let m = Z::Dyn::nrows(&z);
    if m == 0 {
        return;
    }

    unsafe {
        match layout {
            VecLayoutTransform::None => {
                if Z::Dyn::is_contiguous(&z) {
                    annotate_noalias_col_with_index::<Z, _>(
                        &mut f,
                        Z::Dyn::get_slice_unchecked(&mut z, 0, m),
                        0,
                        m,
                        false,
                    );
                } else {
                    for i in 0..m {
                        f(Z::from_dyn_idx(i), Z::Dyn::get_unchecked(&mut z, i))
                    }
                }
            }
            VecLayoutTransform::Reverse => {
                if Z::Dyn::is_contiguous(&z) {
                    annotate_noalias_col_with_index::<Z, _>(
                        &mut f,
                        Z::Dyn::get_slice_unchecked(&mut z, 0, m),
                        0,
                        m,
                        true,
                    );
                } else {
                    for i in 0..m {
                        f(Z::from_dyn_idx(m - i - 1), Z::Dyn::get_unchecked(&mut z, i))
                    }
                }
            }
        }
    }
}

#[inline(always)]
fn for_each_row_with_index<
    Idx,
    Z: MatIndex<
        LayoutTransform = VecLayoutTransform,
        Index = Idx,
        Dyn: MatIndex<Rows = Unit, Cols = usize, Index = usize, Item = Z::Item, Slice = Z::Slice>,
    >,
>(
    z: Z,
    mut f: impl FnMut(Idx, <Z as MatIndex>::Item),
) {
    let layout = Z::preferred_layout(&z);
    let mut z = Z::with_layout(z, layout);

    let n = Z::Dyn::ncols(&z);
    if n == 0 {
        return;
    }

    unsafe {
        match layout {
            VecLayoutTransform::None => {
                if Z::Dyn::is_contiguous(&z) {
                    annotate_noalias_col_with_index::<Z, _>(
                        &mut f,
                        Z::Dyn::get_slice_unchecked(&mut z, 0, n),
                        0,
                        n,
                        false,
                    );
                } else {
                    for i in 0..n {
                        f(Z::from_dyn_idx(i), Z::Dyn::get_unchecked(&mut z, i))
                    }
                }
            }
            VecLayoutTransform::Reverse => {
                if Z::Dyn::is_contiguous(&z) {
                    annotate_noalias_col_with_index::<Z, _>(
                        &mut f,
                        Z::Dyn::get_slice_unchecked(&mut z, 0, n),
                        0,
                        n,
                        true,
                    );
                } else {
                    for i in 0..n {
                        f(Z::from_dyn_idx(n - i - 1), Z::Dyn::get_unchecked(&mut z, i))
                    }
                }
            }
        }
    }
}
#[inline(always)]
fn for_each_row<
    Z: MatIndex<
        Dyn: MatIndex<Rows = Unit, Cols = usize, Index = usize, Item = Z::Item, Slice = Z::Slice>,
    >,
>(
    z: Z,
    mut f: impl FnMut(<Z as MatIndex>::Item),
) {
    let layout = Z::preferred_layout(&z);
    let mut z = Z::with_layout(z, layout);

    let n = Z::Dyn::ncols(&z);
    if n == 0 {
        return;
    }

    unsafe {
        if Z::Dyn::is_contiguous(&z) {
            annotate_noalias_col::<Z::Dyn>(&mut f, Z::Dyn::get_slice_unchecked(&mut z, 0, n), 0, n);
        } else {
            for j in 0..n {
                f(Z::Dyn::get_unchecked(&mut z, j))
            }
        }
    }
}

impl<
        Rows: Shape,
        Cols: Shape,
        M: MatIndex<
            LayoutTransform = MatLayoutTransform,
            Rows = Rows,
            Cols = Cols,
            Index = (Idx<Rows>, Idx<Cols>),
            Dyn: MatIndex<Rows = usize, Cols = usize, Index = (usize, usize)>,
        >,
    > LastEq<Rows, Cols, M>
{
    /// Applies `f` to each element of `self`.
    #[inline(always)]
    pub fn for_each(self, f: impl FnMut(<Self as MatIndex>::Item)) {
        for_each_mat(self, f);
    }

    /// Applies `f` to each element of `self`, while passing the indices of the position of the
    /// current element.
    #[inline(always)]
    pub fn for_each_with_index(
        self,
        f: impl FnMut(Idx<Rows>, Idx<Cols>, <Self as MatIndex>::Item),
    ) {
        for_each_mat_with_index(self, f);
    }

    /// Applies `f` to each element of the lower triangular half of `self`, while passing the
    /// indices of the position of the current element.
    ///
    /// `diag` specifies whether the diagonal should be included or excluded.
    #[inline(always)]
    pub fn for_each_triangular_lower_with_index(
        self,
        diag: Diag,
        f: impl FnMut(Idx<Rows>, Idx<Cols>, <Self as MatIndex>::Item),
    ) {
        for_each_mat_triangular_lower_with_index(self, diag, f);
    }

    /// Applies `f` to each element of the upper triangular half of `self`, while passing the
    /// indices of the position of the current element.
    ///
    /// `diag` specifies whether the diagonal should be included or excluded.
    #[inline(always)]
    pub fn for_each_triangular_upper_with_index(
        self,
        diag: Diag,
        f: impl FnMut(Idx<Rows>, Idx<Cols>, <Self as MatIndex>::Item),
    ) {
        for_each_mat_triangular_upper_with_index(self, diag, f);
    }

    /// Applies `f` to each element of the lower triangular half of `self`.
    ///
    /// `diag` specifies whether the diagonal should be included or excluded.
    #[inline(always)]
    pub fn for_each_triangular_lower(self, diag: Diag, f: impl FnMut(<Self as MatIndex>::Item)) {
        for_each_mat_triangular_lower(self, diag, false, f);
    }

    /// Applies `f` to each element of the upper triangular half of `self`.
    ///
    /// `diag` specifies whether the diagonal should be included or excluded.
    #[inline(always)]
    pub fn for_each_triangular_upper(self, diag: Diag, f: impl FnMut(<Self as MatIndex>::Item)) {
        for_each_mat_triangular_lower(self, diag, true, f);
    }

    /// Applies `f` to each element of `self` and collect its result into a new matrix.
    #[inline(always)]
    pub fn map<T>(
        self,
        f: impl FnMut(<Self as MatIndex>::Item) -> T,
    ) -> MatGeneric<Unit, T, Rows, Cols> {
        let (m, n) = (Self::nrows(&self), Self::ncols(&self));
        let mut f = f;
        let mut this = self;
        MatGeneric::from_fn(
            m,
            n,
            #[inline(always)]
            |i, j| f(unsafe { Self::get_unchecked(&mut this, (i, j)) }),
        )
    }

    /// Applies `f` to each element of `self` and collect its result into a new matrix.
    #[inline(always)]
    pub fn map_in<C: Container, T>(
        self,
        f: impl FnMut(<Self as MatIndex>::Item) -> C::Of<T>,
    ) -> MatGeneric<C, T, Rows, Cols> {
        let (m, n) = (Self::nrows(&self), Self::ncols(&self));
        let mut f = f;
        let mut this = self;
        MatGeneric::from_fn(
            m,
            n,
            #[inline(always)]
            |i, j| f(unsafe { Self::get_unchecked(&mut this, (i, j)) }),
        )
    }

    /// Applies `f` to each element of `self` and collect its result into a new matrix.
    #[inline(always)]
    pub fn map_with_index<T>(
        self,
        f: impl FnMut(Idx<Rows>, Idx<Cols>, <Self as MatIndex>::Item) -> T,
    ) -> MatGeneric<Unit, T, Rows, Cols> {
        let (m, n) = (Self::nrows(&self), Self::ncols(&self));
        let mut f = f;
        let mut this = self;

        MatGeneric::from_fn(
            m,
            n,
            #[inline(always)]
            |i, j| f(i, j, unsafe { Self::get_unchecked(&mut this, (i, j)) }),
        )
    }

    /// Applies `f` to each element of `self` and collect its result into a new matrix.
    #[inline(always)]
    pub fn map_with_index_in<C: Container, T>(
        self,
        f: impl FnMut(Idx<Rows>, Idx<Cols>, <Self as MatIndex>::Item) -> C::Of<T>,
    ) -> MatGeneric<C, T, Rows, Cols> {
        let (m, n) = (Self::nrows(&self), Self::ncols(&self));
        let mut f = f;
        let mut this = self;

        MatGeneric::from_fn(
            m,
            n,
            #[inline(always)]
            |i, j| f(i, j, unsafe { Self::get_unchecked(&mut this, (i, j)) }),
        )
    }
}

impl<
        Rows: Shape,
        Cols: Shape,
        L: MatIndex<
            LayoutTransform = MatLayoutTransform,
            Rows = Rows,
            Cols = Cols,
            Index = (Idx<Rows>, Idx<Cols>),
            Dyn: MatIndex<Rows = usize, Cols = usize, Index = (usize, usize)>,
        >,
        R: MatIndex<
            LayoutTransform = MatLayoutTransform,
            Rows = Rows,
            Cols = Cols,
            Index = (Idx<Rows>, Idx<Cols>),
            Dyn: MatIndex<Rows = usize, Cols = usize, Index = (usize, usize)>,
        >,
    > ZipEq<Rows, Cols, L, R>
{
    /// Applies `f` to each element of `self`.
    #[inline(always)]
    pub fn for_each(self, f: impl FnMut(<Self as MatIndex>::Item)) {
        for_each_mat(self, f);
    }

    /// Applies `f` to each element of `self`, while passing the indices of the position of the
    /// current element.
    #[inline(always)]
    pub fn for_each_with_index(
        self,
        f: impl FnMut(Idx<Rows>, Idx<Cols>, <Self as MatIndex>::Item),
    ) {
        for_each_mat_with_index(self, f);
    }

    /// Applies `f` to each element of the lower triangular half of `self`, while passing the
    /// indices of the position of the current element.
    ///
    /// `diag` specifies whether the diagonal should be included or excluded.
    #[inline(always)]
    pub fn for_each_triangular_lower_with_index(
        self,
        diag: Diag,
        f: impl FnMut(Idx<Rows>, Idx<Cols>, <Self as MatIndex>::Item),
    ) {
        for_each_mat_triangular_lower_with_index(self, diag, f);
    }

    /// Applies `f` to each element of the upper triangular half of `self`, while passing the
    /// indices of the position of the current element.
    ///
    /// `diag` specifies whether the diagonal should be included or excluded.
    #[inline(always)]
    pub fn for_each_triangular_upper_with_index(
        self,
        diag: Diag,
        f: impl FnMut(Idx<Rows>, Idx<Cols>, <Self as MatIndex>::Item),
    ) {
        for_each_mat_triangular_upper_with_index(self, diag, f);
    }

    /// Applies `f` to each element of the lower triangular half of `self`.
    ///
    /// `diag` specifies whether the diagonal should be included or excluded.
    #[inline(always)]
    pub fn for_each_triangular_lower(self, diag: Diag, f: impl FnMut(<Self as MatIndex>::Item)) {
        for_each_mat_triangular_lower(self, diag, false, f);
    }

    /// Applies `f` to each element of the upper triangular half of `self`.
    ///
    /// `diag` specifies whether the diagonal should be included or excluded.
    #[inline(always)]
    pub fn for_each_triangular_upper(self, diag: Diag, f: impl FnMut(<Self as MatIndex>::Item)) {
        for_each_mat_triangular_lower(self, diag, true, f);
    }

    /// Applies `f` to each element of `self` and collect its result into a new matrix.
    #[inline(always)]
    pub fn map<T>(
        self,
        f: impl FnMut(<Self as MatIndex>::Item) -> T,
    ) -> MatGeneric<Unit, T, Rows, Cols> {
        let (m, n) = (Self::nrows(&self), Self::ncols(&self));
        let mut f = f;
        let mut this = self;
        MatGeneric::from_fn(
            m,
            n,
            #[inline(always)]
            |i, j| f(unsafe { Self::get_unchecked(&mut this, (i, j)) }),
        )
    }

    /// Applies `f` to each element of `self` and collect its result into a new matrix.
    #[inline(always)]
    pub fn map_with_index<T>(
        self,
        f: impl FnMut(Idx<Rows>, Idx<Cols>, <Self as MatIndex>::Item) -> T,
    ) -> MatGeneric<Unit, T, Rows, Cols> {
        let (m, n) = (Self::nrows(&self), Self::ncols(&self));
        let mut f = f;
        let mut this = self;

        MatGeneric::from_fn(
            m,
            n,
            #[inline(always)]
            |i, j| f(i, j, unsafe { Self::get_unchecked(&mut this, (i, j)) }),
        )
    }

    /// Applies `f` to each element of `self` and collect its result into a new matrix.
    #[inline(always)]
    pub fn map_in<C: Container, T>(
        self,
        f: impl FnMut(<Self as MatIndex>::Item) -> C::Of<T>,
    ) -> MatGeneric<C, T, Rows, Cols> {
        let (m, n) = (Self::nrows(&self), Self::ncols(&self));
        let mut f = f;
        let mut this = self;
        MatGeneric::from_fn(
            m,
            n,
            #[inline(always)]
            |i, j| f(unsafe { Self::get_unchecked(&mut this, (i, j)) }),
        )
    }

    /// Applies `f` to each element of `self` and collect its result into a new matrix.
    #[inline(always)]
    pub fn map_with_index_in<C: Container, T>(
        self,
        f: impl FnMut(Idx<Rows>, Idx<Cols>, <Self as MatIndex>::Item) -> C::Of<T>,
    ) -> MatGeneric<C, T, Rows, Cols> {
        let (m, n) = (Self::nrows(&self), Self::ncols(&self));
        let mut f = f;
        let mut this = self;

        MatGeneric::from_fn(
            m,
            n,
            #[inline(always)]
            |i, j| f(i, j, unsafe { Self::get_unchecked(&mut this, (i, j)) }),
        )
    }
}

impl<
        Rows: Shape,
        M: MatIndex<
            LayoutTransform = VecLayoutTransform,
            Rows = Rows,
            Cols = Unit,
            Index = Idx<Rows>,
            Dyn: MatIndex<Rows = usize, Cols = Unit, Index = usize>,
        >,
    > LastEq<Rows, Unit, M>
{
    /// Applies `f` to each element of `self`.
    #[inline(always)]
    pub fn for_each(self, f: impl FnMut(<Self as MatIndex>::Item)) {
        for_each_col(self, f);
    }

    /// Applies `f` to each element of `self`, while passing the indices of the position of the
    /// current element.
    #[inline(always)]
    pub fn for_each_with_index(self, f: impl FnMut(Idx<Rows>, <Self as MatIndex>::Item)) {
        for_each_col_with_index(self, f);
    }

    /// Applies `f` to each element of `self` and collect its result into a new matrix.
    #[inline(always)]
    pub fn map<T>(self, f: impl FnMut(<Self as MatIndex>::Item) -> T) -> ColGeneric<Unit, T, Rows> {
        let (m, _) = (Self::nrows(&self), Self::ncols(&self));
        let mut f = f;
        let mut this = self;
        ColGeneric::from_fn(
            m,
            #[inline(always)]
            |i| f(unsafe { Self::get_unchecked(&mut this, i) }),
        )
    }

    /// Applies `f` to each element of `self` and collect its result into a new matrix.
    #[inline(always)]
    pub fn map_with_index<T>(
        self,
        f: impl FnMut(Idx<Rows>, <Self as MatIndex>::Item) -> T,
    ) -> ColGeneric<Unit, T, Rows> {
        let (m, _) = (Self::nrows(&self), Self::ncols(&self));
        let mut f = f;
        let mut this = self;

        ColGeneric::from_fn(
            m,
            #[inline(always)]
            |i| f(i, unsafe { Self::get_unchecked(&mut this, i) }),
        )
    }

    /// Applies `f` to each element of `self` and collect its result into a new matrix.
    #[inline(always)]
    pub fn map_in<C: Container, T>(
        self,
        f: impl FnMut(<Self as MatIndex>::Item) -> C::Of<T>,
    ) -> ColGeneric<C, T, Rows> {
        let (m, _) = (Self::nrows(&self), Self::ncols(&self));
        let mut f = f;
        let mut this = self;
        ColGeneric::from_fn(
            m,
            #[inline(always)]
            |i| f(unsafe { Self::get_unchecked(&mut this, i) }),
        )
    }

    /// Applies `f` to each element of `self` and collect its result into a new matrix.
    #[inline(always)]
    pub fn map_with_index_in<C: Container, T>(
        self,
        f: impl FnMut(Idx<Rows>, <Self as MatIndex>::Item) -> C::Of<T>,
    ) -> ColGeneric<C, T, Rows> {
        let (m, _) = (Self::nrows(&self), Self::ncols(&self));
        let mut f = f;
        let mut this = self;

        ColGeneric::from_fn(
            m,
            #[inline(always)]
            |i| f(i, unsafe { Self::get_unchecked(&mut this, i) }),
        )
    }
}

impl<
        Rows: Shape,
        L: MatIndex<
            LayoutTransform = VecLayoutTransform,
            Rows = Rows,
            Cols = Unit,
            Index = Idx<Rows>,
            Dyn: MatIndex<Rows = usize, Cols = Unit, Index = usize>,
        >,
        R: MatIndex<
            LayoutTransform = VecLayoutTransform,
            Rows = Rows,
            Cols = Unit,
            Index = Idx<Rows>,
            Dyn: MatIndex<Rows = usize, Cols = Unit, Index = usize>,
        >,
    > ZipEq<Rows, Unit, L, R>
{
    /// Applies `f` to each element of `self`.
    #[inline(always)]
    pub fn for_each(self, f: impl FnMut(<Self as MatIndex>::Item)) {
        for_each_col(self, f);
    }

    /// Applies `f` to each element of `self`, while passing the indices of the position of the
    /// current element.
    #[inline(always)]
    pub fn for_each_with_index(self, f: impl FnMut(Idx<Rows>, <Self as MatIndex>::Item)) {
        for_each_col_with_index(self, f);
    }

    /// Applies `f` to each element of `self` and collect its result into a new matrix.
    #[inline(always)]
    pub fn map<T>(self, f: impl FnMut(<Self as MatIndex>::Item) -> T) -> ColGeneric<Unit, T, Rows> {
        let (m, _) = (Self::nrows(&self), Self::ncols(&self));
        let mut f = f;
        let mut this = self;
        ColGeneric::from_fn(
            m,
            #[inline(always)]
            |i| f(unsafe { Self::get_unchecked(&mut this, i) }),
        )
    }

    /// Applies `f` to each element of `self` and collect its result into a new matrix.
    #[inline(always)]
    pub fn map_with_index<T>(
        self,
        f: impl FnMut(Idx<Rows>, <Self as MatIndex>::Item) -> T,
    ) -> ColGeneric<Unit, T, Rows> {
        let (m, _) = (Self::nrows(&self), Self::ncols(&self));
        let mut f = f;
        let mut this = self;

        ColGeneric::from_fn(
            m,
            #[inline(always)]
            |i| f(i, unsafe { Self::get_unchecked(&mut this, i) }),
        )
    }

    /// Applies `f` to each element of `self` and collect its result into a new matrix.
    #[inline(always)]
    pub fn map_in<C: Container, T>(
        self,
        f: impl FnMut(<Self as MatIndex>::Item) -> C::Of<T>,
    ) -> ColGeneric<C, T, Rows> {
        let (m, _) = (Self::nrows(&self), Self::ncols(&self));
        let mut f = f;
        let mut this = self;
        ColGeneric::from_fn(
            m,
            #[inline(always)]
            |i| f(unsafe { Self::get_unchecked(&mut this, i) }),
        )
    }

    /// Applies `f` to each element of `self` and collect its result into a new matrix.
    #[inline(always)]
    pub fn map_with_index_in<C: Container, T>(
        self,
        f: impl FnMut(Idx<Rows>, <Self as MatIndex>::Item) -> C::Of<T>,
    ) -> ColGeneric<C, T, Rows> {
        let (m, _) = (Self::nrows(&self), Self::ncols(&self));
        let mut f = f;
        let mut this = self;

        ColGeneric::from_fn(
            m,
            #[inline(always)]
            |i| f(i, unsafe { Self::get_unchecked(&mut this, i) }),
        )
    }
}

impl<
        Cols: Shape,
        M: MatIndex<
            LayoutTransform = VecLayoutTransform,
            Rows = Unit,
            Cols = Cols,
            Index = Idx<Cols>,
            Dyn: MatIndex<Rows = Unit, Cols = usize, Index = usize>,
        >,
    > LastEq<Unit, Cols, M>
{
    /// Applies `f` to each element of `self`.
    #[inline(always)]
    pub fn for_each(self, f: impl FnMut(<Self as MatIndex>::Item)) {
        for_each_row(self, f);
    }

    /// Applies `f` to each element of `self`, while passing the indices of the position of the
    /// current element.
    #[inline(always)]
    pub fn for_each_with_index(self, f: impl FnMut(Idx<Cols>, <Self as MatIndex>::Item)) {
        for_each_row_with_index(self, f);
    }

    /// Applies `f` to each element of `self` and collect its result into a new matrix.
    #[inline(always)]
    pub fn map<T>(self, f: impl FnMut(<Self as MatIndex>::Item) -> T) -> RowGeneric<Unit, T, Cols> {
        let (_, n) = (Self::nrows(&self), Self::ncols(&self));
        let mut f = f;
        let mut this = self;
        RowGeneric::from_fn(
            n,
            #[inline(always)]
            |i| f(unsafe { Self::get_unchecked(&mut this, i) }),
        )
    }

    /// Applies `f` to each element of `self` and collect its result into a new matrix.
    #[inline(always)]
    pub fn map_with_index<T>(
        self,
        f: impl FnMut(Idx<Cols>, <Self as MatIndex>::Item) -> T,
    ) -> RowGeneric<Unit, T, Cols> {
        let (_, n) = (Self::nrows(&self), Self::ncols(&self));
        let mut f = f;
        let mut this = self;

        RowGeneric::from_fn(
            n,
            #[inline(always)]
            |i| f(i, unsafe { Self::get_unchecked(&mut this, i) }),
        )
    }

    /// Applies `f` to each element of `self` and collect its result into a new matrix.
    #[inline(always)]
    pub fn map_in<C: Container, T>(
        self,
        f: impl FnMut(<Self as MatIndex>::Item) -> C::Of<T>,
    ) -> RowGeneric<C, T, Cols> {
        let (_, n) = (Self::nrows(&self), Self::ncols(&self));
        let mut f = f;
        let mut this = self;
        RowGeneric::from_fn(
            n,
            #[inline(always)]
            |i| f(unsafe { Self::get_unchecked(&mut this, i) }),
        )
    }

    /// Applies `f` to each element of `self` and collect its result into a new matrix.
    #[inline(always)]
    pub fn map_with_index_in<C: Container, T>(
        self,
        f: impl FnMut(Idx<Cols>, <Self as MatIndex>::Item) -> C::Of<T>,
    ) -> RowGeneric<C, T, Cols> {
        let (_, n) = (Self::nrows(&self), Self::ncols(&self));
        let mut f = f;
        let mut this = self;

        RowGeneric::from_fn(
            n,
            #[inline(always)]
            |i| f(i, unsafe { Self::get_unchecked(&mut this, i) }),
        )
    }
}

impl<
        Cols: Shape,
        L: MatIndex<
            LayoutTransform = VecLayoutTransform,
            Rows = Unit,
            Cols = Cols,
            Index = Idx<Cols>,
            Dyn: MatIndex<Rows = Unit, Cols = usize, Index = usize>,
        >,
        R: MatIndex<
            LayoutTransform = VecLayoutTransform,
            Rows = Unit,
            Cols = Cols,
            Index = Idx<Cols>,
            Dyn: MatIndex<Rows = Unit, Cols = usize, Index = usize>,
        >,
    > ZipEq<Unit, Cols, L, R>
{
    /// Applies `f` to each element of `self`.
    #[inline(always)]
    pub fn for_each(self, f: impl FnMut(<Self as MatIndex>::Item)) {
        for_each_row(self, f);
    }

    /// Applies `f` to each element of `self`, while passing the indices of the position of the
    /// current element.
    #[inline(always)]
    pub fn for_each_with_index(self, f: impl FnMut(Idx<Cols>, <Self as MatIndex>::Item)) {
        for_each_row_with_index(self, f);
    }

    /// Applies `f` to each element of `self` and collect its result into a new matrix.
    #[inline(always)]
    pub fn map<T>(self, f: impl FnMut(<Self as MatIndex>::Item) -> T) -> RowGeneric<Unit, T, Cols> {
        let (_, n) = (Self::nrows(&self), Self::ncols(&self));
        let mut f = f;
        let mut this = self;
        RowGeneric::from_fn(
            n,
            #[inline(always)]
            |i| f(unsafe { Self::get_unchecked(&mut this, i) }),
        )
    }

    /// Applies `f` to each element of `self` and collect its result into a new matrix.
    #[inline(always)]
    pub fn map_with_index<T>(
        self,
        f: impl FnMut(Idx<Cols>, <Self as MatIndex>::Item) -> T,
    ) -> RowGeneric<Unit, T, Cols> {
        let (_, n) = (Self::nrows(&self), Self::ncols(&self));
        let mut f = f;
        let mut this = self;

        RowGeneric::from_fn(
            n,
            #[inline(always)]
            |i| f(i, unsafe { Self::get_unchecked(&mut this, i) }),
        )
    }

    /// Applies `f` to each element of `self` and collect its result into a new matrix.
    #[inline(always)]
    pub fn map_in<C: Container, T>(
        self,
        f: impl FnMut(<Self as MatIndex>::Item) -> C::Of<T>,
    ) -> RowGeneric<C, T, Cols> {
        let (_, n) = (Self::nrows(&self), Self::ncols(&self));
        let mut f = f;
        let mut this = self;
        RowGeneric::from_fn(
            n,
            #[inline(always)]
            |i| f(unsafe { Self::get_unchecked(&mut this, i) }),
        )
    }

    /// Applies `f` to each element of `self` and collect its result into a new matrix.
    #[inline(always)]
    pub fn map_with_index_in<C: Container, T>(
        self,
        f: impl FnMut(Idx<Cols>, <Self as MatIndex>::Item) -> C::Of<T>,
    ) -> RowGeneric<C, T, Cols> {
        let (_, n) = (Self::nrows(&self), Self::ncols(&self));
        let mut f = f;
        let mut this = self;

        RowGeneric::from_fn(
            n,
            #[inline(always)]
            |i| f(i, unsafe { Self::get_unchecked(&mut this, i) }),
        )
    }
}
