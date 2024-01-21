package main

import "fmt"

type Age int

func (age *Age)add(num *Age) {
    *age += *num
}

func main() {
    age := Age(0)
    age := Age(10)
    age.add(&Age(18)
    fmt.Println(age)
}