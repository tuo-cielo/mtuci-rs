/*
----> ЗАДАНИЕ 2 "Площадь квадрата"

Создать структуру Rect (квадрат), которая задается координатами левого верхнего угла и длиной стороны.
Добавить для этой структуры методы new(top_left: (f32, f32), width: f32) -> Rect
                                   bottom_right -> (f32, f32), // Выводит координаты правого нижнего угла
                                   area -> f32 // Вычисляет площадь квадрата
                                   perimeter -> f32 // Вычисляет периметр квадрата

 */


fn main() {
    let rect = Rect::new((2.0, 4.0), 3.0);
    println!("{:?}", rect.bottom_right());
    println!("{}", rect.area());
    println!("{}", rect.perimeter());
}

struct Rect {
    top_left: (f32, f32),
    width: f32
}

impl Rect {
    fn new(top_left: (f32, f32), width: f32) -> Self {
        Rect { top_left: (top_left.0, top_left.1), width }
    }

    fn bottom_right(&self) -> (f32, f32) {
        (self.top_left.0 + self.width, self.top_left.1 - self.width)
    }

    fn area(&self) -> f32 {
        self.width.powf(2.0)
    }

    fn perimeter(&self) -> f32 {
        self.width * 4.0
    }
}


// ----> TESTS
#[cfg(test)]
mod tests {
    use crate::Rect;

    #[test]
    fn bottom_right() {
        let rect = Rect::new((1., 2.), 5.);

        assert_eq!((6., -3.), rect.bottom_right())
    }

    #[test]
    fn area() {
        let rect = Rect::new((1., 2.), 5.);

        assert_eq!(25., rect.area())
    }

    #[test]
    fn perimeter() {
        let rect = Rect::new((1., 2.), 5.);

        assert_eq!(20., rect.perimeter())
    }
}