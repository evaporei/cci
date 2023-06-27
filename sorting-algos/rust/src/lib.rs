pub fn is_sorted<T: Ord>(l: &[T]) -> bool {
    l.windows(2).all(|w| w[0] <= w[1])
}

pub fn bubble_sort<T: Ord>(l: &mut [T]) {
    for _ in 0..l.len() - 1 {
        for i in 0..l.len() - 1 {
            if l.get(i) > l.get(i + 1) {
                l.swap(i, i + 1);
            }
        }
    }
}

pub fn bubble_sort2<T: Ord>(l: &mut [T]) {
    let mut sorted;

    loop {
        sorted = false;
        for i in 0..l.len() - 1 {
            if l[i] > l[i + 1] {
                l.swap(i, i + 1);
                sorted = true;
            }
        }
        if !sorted {
            break;
        }
    }
}

#[test]
fn test_bubble_sort() {
    let mut v = vec![7, 6, 4, 3];
    bubble_sort(&mut v);
    assert_eq!(v, vec![3, 4, 6, 7]);

    let mut v = vec![7, 6, 4, 3];
    bubble_sort2(&mut v);
    assert_eq!(v, vec![3, 4, 6, 7]);
}

pub fn selection_sort<T: Ord>(l: &mut [T]) {
    for j in 0..l.len() {
        let mut smallest = j;

        for i in j + 1..l.len() {
            if l[i] < l[smallest] {
                smallest = i;
            }
        }

        if smallest != j {
            l.swap(smallest, j);
        }
    }
}

#[test]
fn test_selection_sort() {
    let mut v = vec![7, 6, 4, 3];
    selection_sort(&mut v);
    assert_eq!(v, vec![3, 4, 6, 7]);
}

pub fn insertion_sort<T: Ord>(l: &mut [T]) {
    for i in 1..l.len() {
        let mut j = i;
        while j > 0 && l[j] < l[j - 1] {
            l.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[test]
fn test_insertion_sort() {
    let mut v = vec![7, 6, 4, 3];
    insertion_sort(&mut v);
    assert_eq!(v, vec![3, 4, 6, 7]);
}
