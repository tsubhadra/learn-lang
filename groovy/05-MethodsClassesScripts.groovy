def greet(name = "World") {
    "Hello, $name!"
}
println greet()
println greet("Arun")

class Person {
    String name
    int age
}
def p = new Person(name: "Subhadra", age: 25)
println "${p.name} is ${p.age} years old"
