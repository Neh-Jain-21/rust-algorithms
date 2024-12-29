pub struct Comparator<T>
where
    T: std::cmp::PartialEq + std::cmp::PartialOrd + 'static,
{
    compare: Box<dyn Fn(&T, &T) -> i32>,
}

impl<T> Comparator<T>
where
    T: std::cmp::PartialEq + std::cmp::PartialOrd + 'static,
{
    /// Creates a new instance of Comparator.
    pub fn new(compare_function: Option<Box<dyn Fn(&T, &T) -> i32>>) -> Self {
        let compare: Box<dyn Fn(&T, &T) -> i32> =
            compare_function.unwrap_or_else(|| Box::new(Self::default_compare_function));
        Comparator { compare }
    }

    /// Default comparison function. It just assumes that "a" and "b" are strings or numbers.
    pub fn default_compare_function(a: &T, b: &T) -> i32 {
        if a == b {
            0
        } else if a > b {
            1
        } else {
            -1
        }
    }

    /// Checks if two variables are equal.
    pub fn equal(&self, a: &T, b: &T) -> bool {
        (self.compare)(&a, &b) == 0
    }

    /// Checks if variable "a" is less than "b".
    pub fn less_than(&self, a: &T, b: &T) -> bool {
        (self.compare)(&a, &b) < 0
    }

    /// Checks if variable "a" is greater than "b".
    pub fn greater_than(&self, a: &T, b: &T) -> bool {
        (self.compare)(&a, &b) > 0
    }

    /// Checks if variable "a" is less than or equal to "b".
    pub fn less_than_or_equal(&self, a: &T, b: &T) -> bool {
        (self.compare)(&a, &b) <= 0
    }

    /// Checks if variable "a" is greater than or equal to "b".
    pub fn greater_than_or_equal(&self, a: &T, b: &T) -> bool {
        (self.compare)(&a, &b) >= 0
    }

    /// Reverses the comparison order.
    pub fn reverse(self) -> Self {
        Comparator {
            compare: Box::new(move |a: &T, b: &T| (self.compare)(&a, &b) * -1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comparator() {
        let comparator: Comparator<i32> = Comparator::new(None);

        assert!(comparator.equal(&1, &1));
        assert!(comparator.less_than(&1, &2));
        assert!(comparator.greater_than(&2, &1));
        assert!(comparator.less_than_or_equal(&1, &1));
        assert!(comparator.less_than_or_equal(&1, &2));
        assert!(comparator.greater_than_or_equal(&2, &2));

        let custom_comparator: Comparator<i32> =
            Comparator::new(Some(Box::new(|a: &i32, b: &i32| b.cmp(a) as i32)));
        assert!(custom_comparator.less_than(&2, &1));
        let custom_comparator: Comparator<i32> = custom_comparator.reverse();
        assert!(custom_comparator.greater_than(&2, &1));
    }
}
