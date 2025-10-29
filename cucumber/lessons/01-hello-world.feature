# 01-hello-world.feature
# A simple Cucumber feature file to demonstrate "Hello World"
# Written in Gherkin syntax (Cucumber's language)
# Defines a scenario to print "Hello world!"
# Uses Given-When-Then pattern
Feature: Hello World
  As a beginner learning Cucumber
  I want to see a simple Hello World example
  So that I can understand how Cucumber works

  Scenario: Print Hello World message
    Given I have a greeting function
    When I call the greeting function
    Then I should see "Hello world!" printed
