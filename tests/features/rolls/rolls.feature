Feature: Animal feature

  Scenario: We can make skill rolls
    Given the kingdom Aryc at level 1
    And a die roll of 10
    When I roll Arts
    Then I get a result of 14 (natural 10)

