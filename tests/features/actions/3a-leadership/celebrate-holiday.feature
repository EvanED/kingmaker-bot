Feature: Activity Phase, Step 1 (Leadership) -- Celebrate Holiday

    Scenario: Celebrate Holiday critically succeeds
        Given the kingdom Aryc at level 1
        And the kingdom has 10 RP
        And a die roll of 20
        When I Celebrate Holiday
        Then there is a +2 circumstance bonus to Loyalty until the end of the next turn, because "critical success celebrating holiday"
        And RP is still 10

    Scenario: Celebrate Holiday succeeds
        Given the kingdom Aryc at level 1
        And the kingdom has 10 RP
        And a die roll of 15
        And the next d4 rolls are 3
        When I Celebrate Holiday
        Then there is a +1 circumstance bonus to Loyalty until the end of the next turn, because "success celebrating holiday"
        And RP went down to 7

    Scenario: Celebrate Holiday fails
        Given the kingdom Aryc at level 1
        And the kingdom has 10 RP
        And a die roll of 5
        And the next d4 rolls are 3
        When I Celebrate Holiday
        Then RP went down to 7
        And there are no remaining bonuses

    Scenario: Celebrate Holiday critically fails
        Given the kingdom Aryc at level 1
        And the kingdom has 10 RP
        And a die roll of 1
        When I Celebrate Holiday
        Then there is a -1 circumstance penalty to Loyalty until the end of the next turn, because "critical failure celebrating holiday"
        And there is 1 requirement
        And "there is a penalty of 4 resource dice next turn" is a requirement

# TODO: Test cases for when the RP cost on success/failure is unaffordable