plugins {
    kotlin("jvm") version "1.8.10"
}

group = "com.github.teamhartex"
version = "0.1.0"

repositories {
    mavenCentral()
}

dependencies {
    implementation("com.github.ajalt.mordant:mordant:2.0.0-beta11")
    implementation("org.jetbrains.kotlin:kotlin-scripting-jvm:1.8.10")
    implementation("org.jetbrains.kotlin:kotlin-scripting-jvm-host:1.8.10")
    implementation(kotlin("reflect"))
    implementation(project(":script-def"))
}

tasks.withType<Jar> {
    archiveClassifier.set("uber")

    manifest {
        attributes["Main-Class"] = "com.github.teamhartex.hartex.buildsystem.MainKt"
    }

    from(sourceSets.main.get().output)

    dependsOn(configurations.runtimeClasspath)
    from({
        configurations.runtimeClasspath.get().filter { it.name.endsWith("jar") }.map { zipTree(it) }
    })

    duplicatesStrategy = DuplicatesStrategy.EXCLUDE
}
