Feature: Commerce Phase, Step 1 -- Collect Taxes

    Scenario: The collection of taxes is critically successful
        Given the kingdom Aryc at level 1
        And the kingdom did not collect taxes the previous turn
        And we have 1 Unrest
        And a die roll of 20
        When I collect taxes
        Then there is a +2 circumstance bonus to Economy until the end of the turn, because "critical success collecting taxes"
        And Unrest is still 1
        And we collected taxes this turn
        And the roll result was a critical success
        And the roll result was a natural 20
        And the roll result was a total 28

    Scenario: The collection of taxes doesn't increase unrest on a critical success, even if we collected last turn
        Given the kingdom Aryc at level 1
        And the kingdom did collect taxes the previous turn
        And we have 1 Unrest
        And a die roll of 20
        When I collect taxes
        Then Unrest is still 1
        And we collected taxes this turn

    Scenario: The collection of taxes is successful
        Given the kingdom Aryc at level 1
        And the kingdom did not collect taxes the previous turn
        And we have 1 Unrest
        And a die roll of 15
        When I collect taxes
        Then there is a +1 circumstance bonus to Economy until the end of the turn, because "success collecting taxes"
        And Unrest is still 1
        And we collected taxes this turn
        And the roll result was a success
        And the roll result was a natural 15
        And the roll result was a total 23

    Scenario: The collection of taxes is successful, but we collected last turn too
        Given the kingdom Aryc at level 1
        And the kingdom did collect taxes the previous turn
        And we have 1 Unrest
        And a die roll of 15
        When I collect taxes
        Then there is a +1 circumstance bonus to Economy until the end of the turn, because "success collecting taxes"
        And Unrest went up to 2
        And we collected taxes this turn

    Scenario: The collection of taxes is unsuccessful
        Given the kingdom Aryc at level 1
        And the kingdom did not collect taxes the previous turn
        And we have 1 Unrest
        And a die roll of 5
        When I collect taxes
        Then there is a +1 circumstance bonus to Economy until the end of the turn, because "failure collecting taxes"
        And Unrest went up to 2
        And we collected taxes this turn

    Scenario: The collection of taxes is unsuccessful, but we collected last turn too
        Given the kingdom Aryc at level 1
        And the kingdom did collect taxes the previous turn
        And we have 1 Unrest
        And a die roll of 5
        When I collect taxes
        Then there is a +1 circumstance bonus to Economy until the end of the turn, because "failure collecting taxes"
        And Unrest went up to 3
        And we collected taxes this turn

    Scenario: The collection of taxes is critically unsuccessful
        Given the kingdom Aryc at level 1
        And we have 1 Unrest
        And a die roll of 1
        When I collect taxes
        Then there is no bonus
        And Unrest went up to 3
        And we are required to increase any Ruin

    Scenario: We do not collect taxes, and get to decrease Unrest
        Given the kingdom Aryc at level 1
        And we have 2 Unrest
        And a die roll of 11
        When I do not collect taxes
        Then there is no bonus
        And Unrest went down to 1
    
    Scenario: We do not collect taxes, but do not get to decrease Unrest
        Given the kingdom Aryc at level 1
        And we have 2 Unrest
        And a die roll of 10
        When I do not collect taxes
        Then there is no bonus
        And Unrest is still 2
    
     Scenario: We do not collect taxes, would roll high, but there is no Unrest to decrease
        Given the kingdom Aryc at level 1
        And we have 0 Unrest
        And a die roll of 20
        When I do not collect taxes
        Then there is no bonus
        And Unrest is still 0
    
    