use crate::convert::TryFrom;
use crate::{
    intrinsics,
    iter::from_fn,
    ops::{Range, Try},
};

/// An iterator for stepping iterators by a custom amount.
///
/// This `struct` is created by the [`step_by`] method on [`Iterator`]. See
/// its documentation for more.
///
/// [`step_by`]: Iterator::step_by
/// [`Iterator`]: trait.Iterator.html
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[stable(feature = "iterator_step_by", since = "1.28.0")]
#[derive(Clone, Debug)]
pub struct StepBy<I> {
    iter: I,
    step: usize,
    first_take: bool,
}

impl<I> StepBy<I> {
    #[inline]
    pub(in crate::iter) fn new(iter: I, step: usize) -> StepBy<I> {
        assert!(step != 0);
        let iter = <I as SpecRangeSetup<I>>::setup(iter, step);
        StepBy { iter, step: step - 1, first_take: true }
    }
}

#[stable(feature = "iterator_step_by", since = "1.28.0")]
impl<I> Iterator for StepBy<I>
where
    I: Iterator,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.spec_next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.spec_size_hint()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.spec_nth(n)
    }

    fn try_fold<Acc, F, R>(&mut self, acc: Acc, f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Output = Acc>,
    {
        self.spec_try_fold(acc, f)
    }

    #[inline]
    fn fold<Acc, F>(self, acc: Acc, f: F) -> Acc
    where
        F: FnMut(Acc, Self::Item) -> Acc,
    {
        self.spec_fold(acc, f)
    }
}

impl<I> StepBy<I>
where
    I: ExactSizeIterator,
{
    // The zero-based index starting from the end of the iterator of the
    // last element. Used in the `DoubleEndedIterator` implementation.
    fn next_back_index(&self) -> usize {
        let rem = self.iter.len() % (self.step + 1);
        if self.first_take {
            if rem == 0 { self.step } else { rem - 1 }
        } else {
            rem
        }
    }
}

#[stable(feature = "double_ended_step_by_iterator", since = "1.38.0")]
impl<I> DoubleEndedIterator for StepBy<I>
where
    I: DoubleEndedIterator + ExactSizeIterator,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.spec_next_back()
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.spec_nth_back(n)
    }

    fn try_rfold<Acc, F, R>(&mut self, init: Acc, f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Output = Acc>,
    {
        self.spec_try_rfold(init, f)
    }

    #[inline]
    fn rfold<Acc, F>(self, init: Acc, f: F) -> Acc
    where
        Self: Sized,
        F: FnMut(Acc, Self::Item) -> Acc,
    {
        self.spec_rfold(init, f)
    }
}

// StepBy can only make the iterator shorter, so the len will still fit.
#[stable(feature = "iterator_step_by", since = "1.28.0")]
impl<I> ExactSizeIterator for StepBy<I> where I: ExactSizeIterator {}

trait SpecRangeSetup<T> {
    fn setup(inner: T, step: usize) -> T;
}

impl<T> SpecRangeSetup<T> for T {
    #[inline]
    default fn setup(inner: T, _step: usize) -> T {
        inner
    }
}

trait StepByImpl<I> {
    type Item;

    fn spec_next(&mut self) -> Option<Self::Item>;

    fn spec_size_hint(&self) -> (usize, Option<usize>);

    fn spec_nth(&mut self, n: usize) -> Option<Self::Item>;

    fn spec_try_fold<Acc, F, R>(&mut self, acc: Acc, f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Output = Acc>;

    fn spec_fold<Acc, F>(self, acc: Acc, f: F) -> Acc
    where
        F: FnMut(Acc, Self::Item) -> Acc;
}

trait StepByBackImpl<I> {
    type Item;

    fn spec_next_back(&mut self) -> Option<Self::Item>
    where
        I: DoubleEndedIterator + ExactSizeIterator;

    fn spec_nth_back(&mut self, n: usize) -> Option<Self::Item>
    where
        I: DoubleEndedIterator + ExactSizeIterator;

    fn spec_try_rfold<Acc, F, R>(&mut self, init: Acc, f: F) -> R
    where
        I: DoubleEndedIterator + ExactSizeIterator,
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Output = Acc>;

    fn spec_rfold<Acc, F>(self, init: Acc, f: F) -> Acc
    where
        I: DoubleEndedIterator + ExactSizeIterator,
        F: FnMut(Acc, Self::Item) -> Acc;
}

impl<I: Iterator> StepByImpl<I> for StepBy<I> {
    type Item = I::Item;

    #[inline]
    default fn spec_next(&mut self) -> Option<I::Item> {
        let step_size = if self.first_take { 0 } else { self.step };
        self.first_take = false;
        self.iter.nth(step_size)
    }

    #[inline]
    default fn spec_size_hint(&self) -> (usize, Option<usize>) {
        #[inline]
        fn first_size(step: usize) -> impl Fn(usize) -> usize {
            move |n| if n == 0 { 0 } else { 1 + (n - 1) / (step + 1) }
        }

        #[inline]
        fn other_size(step: usize) -> impl Fn(usize) -> usize {
            move |n| n / (step + 1)
        }

        let (low, high) = self.iter.size_hint();

        if self.first_take {
            let f = first_size(self.step);
            (f(low), high.map(f))
        } else {
            let f = other_size(self.step);
            (f(low), high.map(f))
        }
    }

    #[inline]
    default fn spec_nth(&mut self, mut n: usize) -> Option<I::Item> {
        if self.first_take {
            self.first_take = false;
            let first = self.iter.next();
            if n == 0 {
                return first;
            }
            n -= 1;
        }
        // n and self.step are indices, we need to add 1 to get the amount of elements
        // When calling `.nth`, we need to subtract 1 again to convert back to an index
        // step + 1 can't overflow because `.step_by` sets `self.step` to `step - 1`
        let mut step = self.step + 1;
        // n + 1 could overflow
        // thus, if n is usize::MAX, instead of adding one, we call .nth(step)
        if n == usize::MAX {
            self.iter.nth(step - 1);
        } else {
            n += 1;
        }

        // overflow handling
        loop {
            let mul = n.checked_mul(step);
            {
                if intrinsics::likely(mul.is_some()) {
                    return self.iter.nth(mul.unwrap() - 1);
                }
            }
            let div_n = usize::MAX / n;
            let div_step = usize::MAX / step;
            let nth_n = div_n * n;
            let nth_step = div_step * step;
            let nth = if nth_n > nth_step {
                step -= div_n;
                nth_n
            } else {
                n -= div_step;
                nth_step
            };
            self.iter.nth(nth - 1);
        }
    }

    default fn spec_try_fold<Acc, F, R>(&mut self, mut acc: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Output = Acc>,
    {
        #[inline]
        fn nth<I: Iterator>(iter: &mut I, step: usize) -> impl FnMut() -> Option<I::Item> + '_ {
            move || iter.nth(step)
        }

        if self.first_take {
            self.first_take = false;
            match self.iter.next() {
                None => return try { acc },
                Some(x) => acc = f(acc, x)?,
            }
        }
        from_fn(nth(&mut self.iter, self.step)).try_fold(acc, f)
    }

    default fn spec_fold<Acc, F>(mut self, mut acc: Acc, mut f: F) -> Acc
    where
        F: FnMut(Acc, Self::Item) -> Acc,
    {
        #[inline]
        fn nth<I: Iterator>(iter: &mut I, step: usize) -> impl FnMut() -> Option<I::Item> + '_ {
            move || iter.nth(step)
        }

        if self.first_take {
            self.first_take = false;
            match self.iter.next() {
                None => return acc,
                Some(x) => acc = f(acc, x),
            }
        }
        from_fn(nth(&mut self.iter, self.step)).fold(acc, f)
    }
}

impl<I: DoubleEndedIterator + ExactSizeIterator> StepByBackImpl<I> for StepBy<I> {
    type Item = I::Item;

    #[inline]
    default fn spec_next_back(&mut self) -> Option<Self::Item> {
        self.iter.nth_back(self.next_back_index())
    }

    #[inline]
    default fn spec_nth_back(&mut self, n: usize) -> Option<I::Item> {
        // `self.iter.nth_back(usize::MAX)` does the right thing here when `n`
        // is out of bounds because the length of `self.iter` does not exceed
        // `usize::MAX` (because `I: ExactSizeIterator`) and `nth_back` is
        // zero-indexed
        let n = n.saturating_mul(self.step + 1).saturating_add(self.next_back_index());
        self.iter.nth_back(n)
    }

    default fn spec_try_rfold<Acc, F, R>(&mut self, init: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Output = Acc>,
    {
        #[inline]
        fn nth_back<I: DoubleEndedIterator>(
            iter: &mut I,
            step: usize,
        ) -> impl FnMut() -> Option<I::Item> + '_ {
            move || iter.nth_back(step)
        }

        match self.next_back() {
            None => try { init },
            Some(x) => {
                let acc = f(init, x)?;
                from_fn(nth_back(&mut self.iter, self.step)).try_fold(acc, f)
            }
        }
    }

    #[inline]
    default fn spec_rfold<Acc, F>(mut self, init: Acc, mut f: F) -> Acc
    where
        Self: Sized,
        F: FnMut(Acc, I::Item) -> Acc,
    {
        #[inline]
        fn nth_back<I: DoubleEndedIterator>(
            iter: &mut I,
            step: usize,
        ) -> impl FnMut() -> Option<I::Item> + '_ {
            move || iter.nth_back(step)
        }

        match self.next_back() {
            None => init,
            Some(x) => {
                let acc = f(init, x);
                from_fn(nth_back(&mut self.iter, self.step)).fold(acc, f)
            }
        }
    }
}

macro_rules! spec_int_ranges {
    ($($t:ty)*) => ($(

        const _: () = assert!(usize::BITS >= <$t>::BITS);

        impl SpecRangeSetup<Range<$t>> for Range<$t> {
            #[inline]
            fn setup(mut r: Range<$t>, step: usize) -> Range<$t> {
                let inner_len = r.size_hint().0;
                // If step exceeds $t::MAX, then the count will be at most 1 and
                // thus always fit into $t.
                let yield_count = inner_len.div_ceil(step);
                // Turn the range end into an iteration counter
                r.end = yield_count as $t;
                r
            }
        }

        impl StepByImpl<Range<$t>> for StepBy<Range<$t>> {
            #[inline]
            fn spec_next(&mut self) -> Option<$t> {
                // if a step size larger than the type has been specified fall back to
                // t::MAX, in which case remaining will be at most 1.
                // The `+ 1` can't overflow since the constructor substracted 1 from the original value.
                let step = <$t>::try_from(self.step + 1).unwrap_or(<$t>::MAX);
                let remaining = self.iter.end;
                if remaining > 0 {
                    let val = self.iter.start;
                    // this can only overflow during the last step, after which the value
                    // will not be used
                    self.iter.start = val.wrapping_add(step);
                    self.iter.end = remaining - 1;
                    Some(val)
                } else {
                    None
                }
            }

            fn spec_size_hint(&self) -> (usize, Option<usize>) {
                let remaining = self.iter.end as usize;
                (remaining, Some(remaining))
            }

            // The methods below are all copied from the Iterator trait default impls.
            // We have to repeat them here so that the specialization overrides the StepByImpl defaults

            fn spec_nth(&mut self, n: usize) -> Option<Self::Item> {
                self.advance_by(n).ok()?;
                self.next()
            }

            fn spec_try_fold<Acc, F, R>(&mut self, init: Acc, mut f: F) -> R
                where
                    F: FnMut(Acc, Self::Item) -> R,
                    R: Try<Output = Acc>
            {
                let mut accum = init;
                while let Some(x) = self.next() {
                    accum = f(accum, x)?;
                }
                try { accum }
            }

            #[inline]
            fn spec_fold<Acc, F>(self, init: Acc, mut f: F) -> Acc
                where
                    F: FnMut(Acc, Self::Item) -> Acc
            {
                // if a step size larger than the type has been specified fall back to
                // t::MAX, in which case remaining will be at most 1.
                let step = <$t>::try_from(self.step + 1).unwrap_or(<$t>::MAX);
                let remaining = self.iter.end;
                let mut acc = init;
                let mut val = self.iter.start;
                for _ in 0..remaining {
                    acc = f(acc, val);
                    // this can only overflow during the last step, after which the value
                    // will no longer be used
                    val = val.wrapping_add(step);
                }
                acc
            }
        }
    )*)
}

macro_rules! spec_int_ranges_r {
    ($($t:ty)*) => ($(
        const _: () = assert!(usize::BITS >= <$t>::BITS);

        impl StepByBackImpl<Range<$t>> for StepBy<Range<$t>> {

            fn spec_next_back(&mut self) -> Option<Self::Item>
                where Range<$t>: DoubleEndedIterator + ExactSizeIterator,
            {
                let step = (self.step + 1) as $t;
                let remaining = self.iter.end;
                if remaining > 0 {
                    let start = self.iter.start;
                    self.iter.end = remaining - 1;
                    Some(start + step * (remaining - 1))
                } else {
                    None
                }
            }

            // The methods below are all copied from the Iterator trait default impls.
            // We have to repeat them here so that the specialization overrides the StepByImplBack defaults

            fn spec_nth_back(&mut self, n: usize) -> Option<Self::Item>
                where Self: DoubleEndedIterator,
            {
                if self.advance_back_by(n).is_err() {
                    return None;
                }
                self.next_back()
            }

            fn spec_try_rfold<Acc, F, R>(&mut self, init: Acc, mut f: F) -> R
                where
                    Self: DoubleEndedIterator,
                    F: FnMut(Acc, Self::Item) -> R,
                    R: Try<Output = Acc>
            {
                let mut accum = init;
                while let Some(x) = self.next_back() {
                    accum = f(accum, x)?;
                }
                try { accum }
            }

            fn spec_rfold<Acc, F>(mut self, init: Acc, mut f: F) -> Acc
                where
                    Self: DoubleEndedIterator,
                    F: FnMut(Acc, Self::Item) -> Acc
            {
                let mut accum = init;
                while let Some(x) = self.next_back() {
                    accum = f(accum, x);
                }
                accum
            }
        }
    )*)
}

#[cfg(target_pointer_width = "64")]
spec_int_ranges!(u8 u16 u32 u64 usize);
// DoubleEndedIterator requires ExactSizeIterator, which isn't implemented for Range<u64>
#[cfg(target_pointer_width = "64")]
spec_int_ranges_r!(u8 u16 u32 usize);

#[cfg(target_pointer_width = "32")]
spec_int_ranges!(u8 u16 u32 usize);
#[cfg(target_pointer_width = "32")]
spec_int_ranges_r!(u8 u16 u32 usize);

#[cfg(target_pointer_width = "16")]
spec_int_ranges!(u8 u16 usize);
#[cfg(target_pointer_width = "16")]
spec_int_ranges_r!(u8 u16 usize);
