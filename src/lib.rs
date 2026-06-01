#[cfg(feature = "random")]
use rand::{seq::SliceRandom, thread_rng};

fn get_pivot<T: Ord + Clone>(slice: &[T]) -> T {
    // base case
    if slice.len() <= 5 {
        let mut v = slice.to_vec();
        v.sort_unstable();
        return v[slice.len() / 2].clone();
    }

    // create array of size n / 5
    let groups = slice.len().div_ceil(5);
    let mut medians: Vec<T> = Vec::with_capacity(groups);

    // create n/5 groups - find median of each (recursion here)
    // create slices - and for each slide determine the median
    for chunk in slice.chunks(5) {
        medians.push(get_pivot(chunk));
    }

    // find median of medians, aka pivot index
    get_pivot(&medians)
}

fn get_kth_smallest<T: Ord + Clone>(k: usize, slice: &[T]) -> T {
    let pivot = get_pivot(slice);

    let mut left: Vec<T> = Vec::new();
    let mut right: Vec<T> = Vec::new();
    for item in slice.iter() {
        if *item < pivot {
            left.push(item.clone());
        } else if *item > pivot {
            right.push(item.clone());
        }
    }

    if left.len() == k - 1 {
        return pivot;
    } else if left.len() > k - 1 {
        return get_kth_smallest(k, &left);
    } else {
        return get_kth_smallest(k - left.len() - 1, &right);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pivot_of_5() {
        let v = vec![2, 1, 5, 4, 3];
        let pivot = get_pivot(&v);
        assert_eq!(pivot, 3);
    }

    #[test]
    fn pivot_of_4() {
        let v = vec![2, 1, 4, 3];
        let pivot = get_pivot(&v);
        assert_eq!(pivot, 3);
    }

    #[test]
    fn pivot_of_11() {
        let v = vec![11, 2, 1, 5, 6, 4, 3, 7, 9, 10, 8];
        let pivot = get_pivot(&v);
        assert_eq!(pivot, 7);
    }

    #[test]
    fn kth_smallest_of_5() {
        let v = vec![2, 1, 5, 4, 3];
        for i in 1..6 {
            let kth_smallest = get_kth_smallest(i, &v);
            assert_eq!(kth_smallest, i);
        }
    }

    #[test]
    fn kth_smallest_of_11() {
        let v = vec![11, 2, 1, 5, 6, 4, 3, 7, 9, 10, 8];
        for i in 1..12 {
            let kth_smallest = get_kth_smallest(i, &v);
            assert_eq!(kth_smallest, i);
        }
    }
}

#[cfg(feature = "random")]
mod tests_random {
    use crate::get_kth_smallest;
    use rand::{seq::SliceRandom, thread_rng};

    #[test]
    fn kth_smallest_batch() {
        let size = 1000;
        let mut v: Vec<usize> = (1..size).collect();

        // Create the random number generator
        let mut rng = thread_rng();

        // Shuffle the vector in place (O(n) time complexity)
        v.shuffle(&mut rng);

        for i in 1..size {
            let kth_smallest = get_kth_smallest(i, &v);
            assert_eq!(kth_smallest, i);
        }
    }
}
