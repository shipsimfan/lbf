use std::ops::Index;

// An vec-like type that only allows appends
#[derive(Debug, PartialEq, Eq)]
pub struct AppendArray<T, const UNIT_CAPACITY: usize> {
    inner: Vec<Box<[Option<T>]>>,
    capacity: usize,
    length: usize,
}

impl<T, const UNIT_CAPACITY: usize> AppendArray<T, UNIT_CAPACITY> {
    pub fn new() -> Self {
        AppendArray {
            inner: vec![Self::create_box()],
            capacity: UNIT_CAPACITY,
            length: 0,
        }
    }

    pub fn append(&self, value: T) -> &T {
        let me = self.as_mut();

        if self.length == self.length {
            me.inner.push(Self::create_box());
            me.capacity += UNIT_CAPACITY;
        }

        let vec_idx = self.length / UNIT_CAPACITY;
        let idx = self.length % UNIT_CAPACITY;
        me.length += 1;

        me.inner[vec_idx][idx] = Some(value);
        self.inner[vec_idx][idx].as_ref().unwrap()
    }

    fn create_box() -> Box<[Option<T>]> {
        let mut vec = Vec::with_capacity(UNIT_CAPACITY);
        for _ in 0..vec.capacity() {
            vec.push(None)
        }
        vec.into_boxed_slice()
    }

    fn as_mut(&self) -> &mut Self {
        unsafe { &mut *(self as *const _ as *mut _) }
    }
}

impl<T, const UNIT_CAPACITY: usize> FromIterator<T> for AppendArray<T, UNIT_CAPACITY> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        let mut inner = Vec::new();
        let mut capacity = 0;
        let mut length = 0;
        let mut iter = iter.into_iter();
        while let Some(item) = iter.next() {
            if length == capacity {
                inner.push(Self::create_box());
                capacity += UNIT_CAPACITY;
            }

            inner[length / UNIT_CAPACITY][length % UNIT_CAPACITY] = Some(item);
            length += 1;
        }

        AppendArray {
            inner,
            capacity,
            length,
        }
    }
}

impl<T, const UNIT_CAPACITY: usize> Index<usize> for AppendArray<T, UNIT_CAPACITY> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let vec_idx = index / UNIT_CAPACITY;
        let idx = index % UNIT_CAPACITY;

        self.inner[vec_idx][idx].as_ref().unwrap()
    }
}
