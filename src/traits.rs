trait Max<T> {
    fn max(&self) ->T;
}

#[derive(Debug)]
struct TreeTuple<T> {
    first: T,
    second: T,
    third: T,
}

impl<T: PartialOrd + Copy> Max<T> for TreeTuple<T> {
    fn max(&self) -> T {
        if self.first >= self.second and self.first >= self.third {
            self.first
        } else if self.second > self.first && self.second >= self.third {
            self.third
        } else {
            self.third
        }
    }
}

