Feature: Celebrate Holiday

    Scenario: Celebrate Holiday critically succeeds
        Given the kingdom Aryc at level 1
        And the kingdom has 1 Fame point
        And the kingdom is not scheduled to gain a Fame point at the start of next turn
        And the kingdom has 1 RP
        And a die roll of 20
        And the next d4 rolls are 3
        When I Celebrate Holiday
        Then Fame points went up to 2
        And the kingdom will gain one additional Fame point next turn
        And RP went up to 7

    Scenario: Celebrate Holiday succeeds
    Scenario: Celebrate Holiday fails
    Scenario: Celebrate Holiday critically fails
    Scenario: Celebrate Holiday critically fails, and the RP cost is unaffordable
