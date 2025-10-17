# Kotlin Ktor Benchmark Test

This test benchmarks a simple Kotlin application using the Ktor framework.

## Framework
- **Language**: Kotlin
- **Framework**: Ktor 2.3.7
- **Server**: Netty
- **JDK**: 21

## Application
The application is a simple HTTP server that responds with "Hello, World!" on the root endpoint (`/`).

## Running the test

```bash
./gradlew run
```

The server will start on `http://0.0.0.0:8080/`

## Building

```bash
./gradlew build
```

## Structure
- `src/main/kotlin/com/example/Application.kt` - Main application file
- `build.gradle.kts` - Gradle build configuration
- `src/main/resources/logback.xml` - Logging configuration
