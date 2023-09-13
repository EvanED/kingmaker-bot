Feature: Take Charge

    Scenario: Take Charge (Arts) critically succeeds
        Given the kingdom Aryc at level 1
        And the kingdom has 2 RP
        And a die roll of 20
        When I Take Charge with Arts
        Then RP went up to 3
        And there is a +1 circumstance bonus to Arts on the next check, because "critical success on Take Charge"

    Scenario: Take Charge (Arts) succeeds
    Scenario: Take Charge (Arts) fails
    Scenario: Take Charge (Arts) critically fails

    Scenario: Take Charge (Industry) succeeds

# TODO: Prerequisite of not repeating this turn
