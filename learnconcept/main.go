package main

// import he
import (
	"encoding/json"
	"fmt"
	"log"
)

// Avenger represents a single hero
type Avenger struct {
	RealName string `json:"real_name"`
	HeroName string `json:"hero_name"`
	Planet   string `json:"planet"`
	Alive    bool   `json:"alive"`
}

func (a *Avenger) isAlive() {
	a.Alive = true
}

func main1() {
	chao()
	hi := greeting("Quoc")
	fmt.Println(hi)
	list := []int{1, 2, 3, 4}
	addItem(1, 100, 200, 300)
	addItem(100, list...)
	change(list...)
	w, h := rectInfo(100, 200)
	fmt.Println(w, h)
	avengers := []Avenger{
		{
			RealName: "Dr. Bruce Banner",
			HeroName: "Hulk",
			Planet:   "Midgard",
		},
		{
			RealName: "Tony Stark",
			HeroName: "Iron Man",
			Planet:   "Midgard",
		},
		{
			RealName: "Thor Odinson",
			HeroName: "Thor",
			Planet:   "Midgard",
		},
	}

	avengers[1].isAlive()

	jsonBytes, err := json.Marshal(avengers)
	if err != nil {
		log.Fatalln(err)
	}
	fmt.Println(string(jsonBytes))
}

func change(list ...int) {
	list[0] = 999
	fmt.Println(list)
}

func addItem(item int, list ...int) {
	list = append(list, item)
	fmt.Println(list)
}
func chao() {
	fmt.Println("Hello")
}

func greeting(name string) string {
	result := fmt.Sprintf("Hello %s", name)
	return result
}

func rectInfo(w, h int) (width int, height int) {
	return w, h
}
