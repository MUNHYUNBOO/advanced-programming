class InternalClass {
    fun fetch(): String {
        return "get user info"
    }
}

class ExternalClass {
    fun search(): String {
        return "get user info"
    }
}

class Adapter(private val external: ExternalClass) {
    fun fetch(): String {
        return external.search()
    }
}

fun main() {
    val internal = InternalClass()
    println("Internal: ${internal.fetch()}")

    val external = ExternalClass()
    val adapter = Adapter(external)
    println("External through Adapter: ${adapter.fetch()}")
}
