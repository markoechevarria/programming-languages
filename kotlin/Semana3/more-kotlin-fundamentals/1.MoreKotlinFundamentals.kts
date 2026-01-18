// GENERICS, OBEJECTS AND EXTENSIONS

class FillInTheBlankQuestion(
    val questionText: String,
    val answer: String,
    val difficulty: String
)

class TrueOrFalseQuestion(
    val questionText: String,
    val answer: Boolean,
    val difficulty: String
)

class NumericQuestion(
    val questionText: String,
    val answer: Int,
    val difficulty: String
)

class Question<T>(
    val questionText: String,
    val answer: T,
    val difficulty: Difficulty
)

class Question<T>(
    val questionText: String,
    val answer: T,
    val difficulty: Difficulty
)

object StudentProgress {
    var total: Int = 10
    var answered: Int = 3
}

fun main() {
    ...
    println("${StudentProgress.answered} of ${StudentProgress.total} answered.")
}

class Quiz {
    val question1 = Question<String>("Quoth the raven ___", "nevermore", Difficulty.MEDIUM)
    val question2 = Question<Boolean>("The sky is green. True or false", false, Difficulty.EASY)
    val question3 = Question<Int>("How many days are there between full moons?", 28, Difficulty.HARD)

    companion object StudentProgress {
        var total: Int = 10
        var answered: Int = 3
    }
}

class Cookie(
    val name: String,
    val softBaked: Boolean,
    val hasFilling: Boolean,
    val price: Double
)

val cookies = listOf(
    Cookie(
        name = "Chocolate Chip",
        softBaked = false,
        hasFilling = false,
        price = 1.69
    ),
    Cookie(
        name = "Banana Walnut", 
        softBaked = true, 
        hasFilling = false, 
        price = 1.49
    ),
    Cookie(
        name = "Vanilla Creme",
        softBaked = false,
        hasFilling = true,
        price = 1.59
    ),
    Cookie(
        name = "Chocolate Peanut Butter",
        softBaked = false,
        hasFilling = true,
        price = 1.49
    ),
    Cookie(
        name = "Snickerdoodle",
        softBaked = true,
        hasFilling = false,
        price = 1.39
    ),
    Cookie(
        name = "Blueberry Tart",
        softBaked = true,
        hasFilling = true,
        price = 1.79
    ),
    Cookie(
        name = "Sugar and Sprinkles",
        softBaked = false,
        hasFilling = false,
        price = 1.39
    )
)

fun main() {
    cookies.forEach {
        println("Menu item: $it")
    }
}


// SOLUTION PRACTICE 1

data class Evento(
    val titulo: String,
    val descripcion: String? = null,
    val diaActividad: String,
    val duracionMinutos: Int,
)

// SOLUTION PRACTICE 2

enum class DiaActividad {
    MANANA,
    TARDE,
    NOCHE,
}

data class Evento(
    val titulo: String,
    val descripcion: String? = null,
    val diaActividad: DiaActividad,
    val duracionMinutos: Int,
)

// SOLUTION PRACTICE 3

val evento1 = Evento(titulo = "Wake up", descripcion = "Time to get up", diaActividad = DiaActividad.MANANA, duracionMinutos = 0)
val evento2 = Evento(titulo = "Eat breakfast", diaActividad = DiaActividad.MANANA, duracionMinutos = 15)
val evento3 = Evento(titulo = "Learn about Kotlin", diaActividad = DiaActividad.TARDE, duracionMinutos = 30)
val evento4 = Evento(titulo = "Practice Compose", diaActividad = DiaActividad.TARDE, duracionMinutos = 60)
val evento5 = Evento(titulo = "Watch latest DevBytes video", diaActividad = DiaActividad.TARDE, duracionMinutos = 10)
val evento6 = Evento(titulo = "Check out latest Android Jetpack library", diaActividad = DiaActividad.NOCHE, duracionMinutos = 45)

val eventos = mutableListOf<Evento>(evento1, evento2, evento3, evento4, evento5, evento6)

// SOULTION PRACTICE 4

val eventosCortos = eventos.filter { it.duracionMinutos < 60 }
println("Tienes ${eventosCortos.size} eventos cortos.")

// SOULTION PRACTICE 5

val eventosAgrupados = eventos.groupBy { it.diaActividad }
eventosAgrupados.forEach { (diaActividad, eventos) ->
    println("$diaActividad: ${eventos.size} eventos")
}

// SOULTION PRACTICE 6

println("Ultimo evento del dia: ${eventos.last().titulo}")

// SOULTION PRACTICE 7

val Event.durationOfEvent: String
    get() = if (this.durationInMinutes < 60) {
        "short"
    } else {
        "long"
    }
