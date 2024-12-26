Feature: Activity Phase, Step 1 (Leadership) -- Take Charge

    Scenario: Take Charge (Arts) critically succeeds
        Given the kingdom Aryc at level 1
        And the kingdom has 2 RP
        And a die roll of 20
        And Take Charge (Arts) has not been used this turn
        When I Take Charge with Arts
        Then RP went up to 3
        And there is a +1 circumstance bonus to Arts on the next check, because "critical success on Take Charge"
        And Take Charge (Arts) has been used this turn

    Scenario: Take Charge (Arts) succeeds
        Given the kingdom Aryc at level 1
        And the kingdom has 2 RP
        And a die roll of 15
        And Take Charge (Arts) has not been used this turn
        When I Take Charge with Arts
        Then RP went up to 3
        And there is no bonus
        And Take Charge (Arts) has been used this turn

    Scenario: Take Charge (Arts) fails
        Given the kingdom Aryc at level 1
        And the kingdom has 2 RP
        And a die roll of 5
        And Take Charge (Arts) has not been used this turn
        When I Take Charge with Arts
        Then RP is still 2
        And there is no bonus
        And Take Charge (Arts) has been used this turn
        
    Scenario: Take Charge (Arts) critically fails
        Given the kingdom Aryc at level 1
        And the kingdom has 2 RP
        And a die roll of 1
        And Take Charge (Arts) has not been used this turn
        When I Take Charge with Arts
        Then RP is still 2
        And there is a -1 circumstance penalty to Arts on the next check, because "critical failure on Take Charge"
        And Take Charge (Arts) has been used this turn

    Scenario: Take Charge (Industry) barely succeeds  # TODO
    Scenario: Take Charge (Industry) barely fails     # TODO

# TODO: Prerequisite of not repeating this turn