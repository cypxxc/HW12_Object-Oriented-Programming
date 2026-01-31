import 'dart:math' show pi;

mixin Shaper {
  double area();
  double perimeter();

  void info() {
    print('Area: ${area().toStringAsFixed(2)}');
    print('Perimeter: ${perimeter().toStringAsFixed(2)}');
  }
}

class Rectangle implements Shaper {
  double width;
  double height;

  Rectangle({required this.width, required this.height});

  @override
  double area() {
    return width * height;
  }

  @override
  double perimeter() {
    return 2 * (width + height);
  }

  @override
  void info() {
    print('Rectangle: ');
    print('width : $width, height : $height');
    print('Area: ${area().toStringAsFixed(2)}');
    print('Perimeter: ${perimeter().toStringAsFixed(2)}');
  }
}

class Circle with Shaper {
  double radius;

  Circle({required this.radius});

  @override
  double area() {
    return pi * radius * radius;
  }

  @override
  double perimeter() {
    return 2 * pi * radius;
  }

  @override
  void info() {
    print('Circle: ');
    print('radius : $radius');
    print('Area: ${area().toStringAsFixed(2)}');
    print('Perimeter: ${perimeter().toStringAsFixed(2)}');
  }
}

void main() {
  Rectangle rectangle = Rectangle(width: 10, height: 20);
  Circle circle = Circle(radius: 10);

  rectangle.info();
  print('');
  circle.info();
}
