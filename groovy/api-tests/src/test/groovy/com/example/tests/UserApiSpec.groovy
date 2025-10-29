package com.example.tests

import spock.lang.*
import groovy.json.JsonSlurper

class UserApiSpec extends Specification {

    def baseUrl = "https://reqres.in/api"

    def setup() {
        println "Starting API Test..."
    }

    def "Verify GET user API"() {
        given: "An API endpoint for a user"
        def endpoint = "${baseUrl}/users/2"

        when: "We send a GET request"
        def response = new URL(endpoint).openConnection()
        response.setRequestMethod("GET")
        response.connect()
        def statusCode = response.responseCode
        def body = new JsonSlurper().parse(response.inputStream)

        then: "We should get status 200 and correct user data"
        statusCode == 200
        body.data.first_name == "Janet"
        body.data.id == 2
    }
}
