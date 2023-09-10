Feature: Bonus Tracking

  Background:
    Given the kingdom Aryc at level 1
    And a die roll of 9

  Scenario: A bonus that lasts until the next roll is removed
    Given a circumstance bonus of +5 to Culture, because "I feel like it"
    When I roll Arts
    Then there are no remaining bonuses

  Scenario: A temporary bonus but doesn't apply is not removed
    Given a circumstance bonus of +5 to Culture, because "I feel like it"
    When I roll Trade
    Then there this is 1 remaining bonus
    And "I feel like it" is in remaining bonuses

  Scenario: A bonus that lasts until the next turn is not removed
    Given a circumstance bonus of +5 to Culture, lasting until the next turn, because "I feel like it"
    When I roll Arts
    Then there this is 1 remaining bonus
    And "I feel like it" is in remaining bonuses

