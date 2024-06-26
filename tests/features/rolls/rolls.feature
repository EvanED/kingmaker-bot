Feature: Kingdom Rolls

  Scenario: We can make skill rolls
    Given the kingdom Aryc at level 1
    And a die roll of 9
    When I roll Arts
    Then I get a result of 13 (natural 9)
    And the roll description is "9 (nat) + 0 (Culture) + 1 (invested) + 3 (training)"

  Scenario: We can make skill rolls with a bonus
    Given the kingdom Aryc at level 1
    And a die roll of 9
    And a circumstance bonus of +5 to Culture, because "I feel like it"
    When I roll Arts
    Then I get a result of 18 (natural 9)
    And the roll description is "9 (nat) + 0 (Culture) + 1 (invested) + 3 (training) + 5 (I feel like it)"

  Scenario: Not all bonuses necessarily apply
    Given the kingdom Aryc at level 1
    And a die roll of 9
    And a circumstance bonus of +5 to Culture, because "I feel like it"
    When I roll Trade
    Then the roll description is "9 (nat) + 4 (Economy) + 1 (invested) + 3 (training)"
