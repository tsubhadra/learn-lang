def num = 5
if (num > 0) println "Positive"

switch (num) {
    case 1..3: println "Small"; break
    case 4..6: println "Medium"; break
    default: println "Large"
}

(1..3).each { println "Count: $it" }
