package main

import (
	"fmt"
	"math"
)

type Shaper interface {
	area() float64
	perimeter() float64
	info()
}

type Rectangle struct {
	width  float64
	height float64
}

func (r Rectangle) area() float64 {
	return r.width * r.height
}

func (r Rectangle) perimeter() float64 {
	return 2 * (r.width + r.height)
}

func (r Rectangle) info() {
	fmt.Println("Rectangle:")
	fmt.Println("width: ", r.width, "height: ", r.height)
	fmt.Println("Area: ", r.area())
	fmt.Println("Perimeter: ", r.perimeter())
}

type Circle struct {
	radius float64
}

func (c Circle) area() float64 {
	return math.Pi * c.radius * c.radius
}

func (c Circle) perimeter() float64 {
	return 2 * math.Pi * c.radius
}

func (c Circle) info() {
	fmt.Println("Circle:")
	fmt.Println("radius: ", c.radius)
	fmt.Printf("Area: %.2f", c.area())
	fmt.Printf("Perimeter: %.2f", c.perimeter())
}

func infoShaper(s Shaper) {
	fmt.Println("Shape:")
	fmt.Println("Area: ", s.area())
	fmt.Println("Perimeter: ", s.perimeter())
}

func main() {
	r := Rectangle{width: 10, height: 20}
	c := Circle{radius: 10}

	r.info()
	c.info()

	infoShaper(r)
	infoShaper(c)
}
