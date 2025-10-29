def fruits = ["apple", "banana", "cherry"]
fruits.each { println it }

def scores = [Math:90, English:85]
scores.each { subject, mark -> println "$subject -> $mark" }

def doubled = fruits.collect { it.toUpperCase() }
println doubled
