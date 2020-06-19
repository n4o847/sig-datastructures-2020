use rand::prelude::*;
use std::cmp::Ordering;

pub fn quicksort<T, R>(a: &mut [T], rng: &mut R)
where
    T: Ord,
    R: Rng + ?Sized,
{
    quicksort_impl(a, 0, a.len(), rng);
}

fn quicksort_impl<T, R>(a: &mut [T], i: usize, n: usize, rng: &mut R)
where
    T: Ord,
    R: Rng + ?Sized,
{
    if n <= 1 {
        return;
    }
    let mut r = i + rng.gen_range(0, n);
    let mut p = i as isize - 1;
    let mut j = i;
    let mut q = i + n;
    while j < q {
        match a[j].cmp(&a[r]) {
            Ordering::Less => {
                p += 1;
                a.swap(j, p as usize);
                if r == p as usize {
                    r = j;
                } else if r == j {
                    r = p as usize;
                }
                j += 1;
            }
            Ordering::Equal => {
                j += 1;
            }
            Ordering::Greater => {
                q -= 1;
                a.swap(j, q);
                if r == q {
                    r = j;
                } else if r == j {
                    r = q;
                }
            }
        }
    }
    quicksort_impl(a, i, (p - i as isize + 1) as usize, rng);
    quicksort_impl(a, q, n - (q - i), rng);
}

#[cfg(test)]
mod tests {
    use super::quicksort;
    use rand::prelude::*;

    fn is_sorted<T>(data: &[T]) -> bool
    where
        T: Ord,
    {
        data.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn test_quicksort() {
        let mut rng = rand::thread_rng();
        let mut v = (0..256).map(|_| rng.gen_range(0, 100)).collect::<Vec<_>>();
        let mut rng = rand::thread_rng();
        dbg!(v
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(" "));
        quicksort(&mut v, &mut rng);
        dbg!(v
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(" "));
        assert!(is_sorted(&v));
    }
}
