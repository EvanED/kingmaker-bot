Feature: Take Charge

    Scenario: Take Charge (Arts) critically succeeds
        Given the kingdom Aryc at level 1
        And the kingdom has 2 RP
        And a die roll of 20
        When I Take Charge with Arts
        Then RP went up to 3
        And there is a +1 circumstance bonus to Arts on the next check, because "critical success on Take Charge"

    Scenario: Take Charge (Arts) succeeds
        Given the kingdom Aryc at level 1
        And the kingdom has 2 RP
        And a die roll of 15
        When I Take Charge with Arts
        Then RP went up to 3
        And there is no bonus

    Scenario: Take Charge (Arts) fails
        Given the kingdom Aryc at level 1
        And the kingdom has 2 RP
        And a die roll of 5
        When I Take Charge with Arts
        Then RP is still 2
        And there is no bonus
        
    Scenario: Take Charge (Arts) critically fails
        Given the kingdom Aryc at level 1
        And the kingdom has 2 RP
        And a die roll of 1
        When I Take Charge with Arts
        Then RP is still 2
        And there is a -1 circumstance penalty to Arts on the next check, because "critical failure on Take Charge"

    Scenario: Take Charge (Industry) barely succeeds  # TODO
    Scenario: Take Charge (Industry) barely fails     # TODO

# TODO: Prerequisite of not repeating this turn
