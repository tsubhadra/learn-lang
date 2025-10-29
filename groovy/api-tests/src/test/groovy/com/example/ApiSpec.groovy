package com.example

import spock.lang.*
import static io.restassured.RestAssured.*

class ApiSpec extends Specification {

    def setup() {
        baseURI = "https://reqres.in"   // sample API
    }

    def "Test GET user API"() {
        given: "A user ID"
        def userId = 2

        when: "We send a GET request"
        def response = get("/api/users/${userId}")

        then: "We get a successful response"
        response.then().statusCode(200)

        and: "The user's first name is Janet"
        response.jsonPath().getString("data.first_name") == "Janet"
    }

    def "Test POST create user API"() {
        given: "A new user request body"
        def requestBody = [name: "morpheus", job: "leader"]

        when: "We send a POST request"
        def response = given()
            .contentType("application/json")
            .body(requestBody)
            .post("/api/users")

        then: "We get a 201 Created response"
        response.then().statusCode(201)

        and: "Response contains the same name"
        response.jsonPath().getString("name") == "morpheus"
    }
}
