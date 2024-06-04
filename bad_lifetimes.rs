fn main() {
    let arr = [2, 4, 8, 16];

    let mut n = 2;
    let nth = nth_item(&arr, &n);
    let increased = increased_by_first_item(&arr, &mut n);

    let value = {
        let values = TwoValues::new(&arr[3], increased);

        assert_eq!(*values.get_first(), 16);

        values.get_second()
    };

    assert_eq!(*value, 4);
    assert_eq!(*nth, 8);
}

fn nth_item(data: &[usize], n: &usize) -> &usize {
    &data[*n]
}

fn increased_by_first_item(data: &[usize], n: &mut usize) -> &mut usize {
    *n += data[0];
    n
}

struct TwoValues {
    first: &usize,
    second: &usize,
}

impl TwoValues {
    pub fn new(first: &usize, second: &usize) -> Self {
        Self { first, second }
    }

    pub fn get_first(&self) -> &usize {
        self.first
    }

    pub fn get_second(&self) -> &usize {
        self.second
    }
}
