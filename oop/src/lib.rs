pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> Self {
        AveragedCollection {
            list: vec![],
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    use crate::AveragedCollection;

    #[test]
    fn should_update_average_for_add_values() {
        let mut list = AveragedCollection::new();
        list.add(2);
        assert_eq!(list.average(), 2.0);
        list.add(4);
        assert_eq!(list.average(), 3.0);
    }

    #[test]
    fn should_update_average_for_removing_value() {
        let mut list = AveragedCollection::new();
        list.add(4);
        list.add(4);
        list.add(1);
        assert_eq!(list.average(), 3.0);
        list.remove().unwrap();
        assert_eq!(list.average(), 4.0);
    }
}
