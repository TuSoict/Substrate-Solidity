package main

import (
	"fmt"
)


type Student struct {
	name string
}

func (s Student) getName()  string {
	return s.name
}
// value reciever
func (s Student) changeName() {
	s.name = "Kaka"
	fmt.Printf("p2 = %p", &s)
	fmt.Println()
}

// pointer reciever
func (s *Student) changeName2() {
	s.name = "Quoc12"
}


func main() {
	student := Student{"QuocNV"}
	name := student.getName()
	fmt.Printf("p1 = %p", &student)
	fmt.Println() 
	fmt.Println(name)

	student.changeName2()
	fmt.Println(student.name)


}