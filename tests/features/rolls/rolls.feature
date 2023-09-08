Feature: Animal feature

  Scenario: If we feed a hungry cat it will no longer be hungry
    Given a hungry cat
    When I feed the cat
    Then the cat is not hungry

  Scenario: We can make skill rolls
    Given the kingdom Aryc at level 1
    And a die roll of 10
    When I roll Arts
    Then I get a result of 14
