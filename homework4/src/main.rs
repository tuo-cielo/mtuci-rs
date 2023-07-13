struct Vector<T: Copy> {
    data: [(Option<T>, usize); 5],
    length: usize,
}

trait TraitName<T: Copy> {
    fn new() -> Self;
    fn with_capacity(_capacity: usize) -> Self;
    fn push(&mut self, element: T);
    fn pop(&mut self) -> Option<T>;
    fn remove(&mut self, index: usize) -> Option<T>;
    fn get(&self, index: usize) -> Option<&T>;
    fn resize(&mut self, _new_size: usize);
}

impl<T: Copy> TraitName<T> for Vector<T> {
    fn new() -> Self {
        Self {
            data: [(None, 0); 5],
            length: 0,
        }
    }

    fn with_capacity(_capacity: usize) -> Self {
        Self {
            data: [(None, 0); 5],
            length: 0,
        }
    }

     fn get(&self, index: usize) -> Option<&T> {
         if index < self.length {
             self.data[index].0.as_ref()
         } else {
             None
         }
     }

    fn push(&mut self, element: T) {
        if self.length < self.data.len() {
            self.data[self.length] = (Some(element), 0);
            self.length += 1;
        } else {
            self.resize(self.data.len() * 2);
            self.push(element);
        }
    }

    fn pop(&mut self) -> Option<T> {
        if self.length > 0 {
            self.length -= 1;
            self.data[self.length].0.take()
        } else {
            None
        }
    }

     fn resize(&mut self, _new_size: usize) {
        let mut new_data: [(Option<T>, usize); 5] = [(None, 0); 5];
        for i in 0..self.length {
            new_data[i].0 = self.data[i].0.take();
        }
        self.data = new_data;
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.length {
            let removed = self.data[index].0.take();
            for i in index..self.length - 1 {
                self.data[i].0 = self.data[i + 1].0.take();
            }
            self.length -= 1;
            removed
        } else {
            None
        }
    }



}

fn main() {
    let mut vector: Vector<i32> = MyVector::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);
    println!("{:?}", vector.pop());
    println!("{:?}", vector.get(1));
    vector.remove(0);
    println!("{:?}", vector.get(0));
}

//  ๐·°(৹˃̵﹏˂̵৹)°·๐