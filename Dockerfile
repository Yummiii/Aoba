FROM gradle:jdk17-alpine as builder
COPY . /app
WORKDIR /app
RUN gradle --no-daemon --parallel -i --build-cache build

FROM openjdk:17-alpine
COPY --from=builder /app/build/libs/*.jar /app/aoba.jar
WORKDIR /app

EXPOSE 8080
ENTRYPOINT ["java", "-jar", "aoba.jar"]