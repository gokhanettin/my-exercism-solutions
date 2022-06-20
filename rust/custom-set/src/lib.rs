#[derive(Debug)]
pub struct CustomSet<T> {
    container: Vec<T>,
}

impl<T> CustomSet<T>
where
    T: Clone + PartialEq,
{
    pub fn new(input: &[T]) -> Self {
        let mut custom_set = CustomSet {
            container: Vec::new(),
        };
        for elem in input {
            custom_set.add(elem.clone());
        }
        custom_set
    }

    pub fn contains(&self, element: &T) -> bool {
        self.container.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.container.push(element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.container.iter().all(|elem| other.contains(elem))
    }

    pub fn is_empty(&self) -> bool {
        self.container.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.container.iter().all(|elem| !other.contains(elem))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let container: Vec<_> = self
            .container
            .iter()
            .filter(|elem| other.contains(elem))
            .cloned()
            .collect();
        CustomSet { container }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let container: Vec<_> = self
            .container
            .iter()
            .filter(|elem| !other.contains(elem))
            .cloned()
            .collect();
        CustomSet { container }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut union_set = CustomSet::new(&self.container);
        for elem in &other.container {
            union_set.add(elem.clone());
        }
        union_set
    }
}

impl<T> PartialEq for CustomSet<T>
where
    T: Clone + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.is_subset(other) && other.is_subset(self)
    }
}
