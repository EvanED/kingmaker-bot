Feature: Commerce Phase, Step 2 -- Improve Lifestyle

    Scenario: Improve Lifestyle is critically successful
        Given the kingdom Aryc at level 1
        And we have 1 Unrest
        And a die roll of 20
        When I Improve Lifestyle
        Then there is a +2 circumstance bonus to Culture until the end of the turn, because "critical success improving lifestyle"
        And Unrest is still 1

    Scenario: Improve Lifestyle is successful
        Given the kingdom Aryc at level 1
        And we have 1 Unrest
        And a die roll of 15
        When I Improve Lifestyle
        Then there is a +1 circumstance bonus to Culture until the end of the turn, because "success improving lifestyle"
        And Unrest is still 1

    Scenario: Improve Lifestyle is a failure
        Given the kingdom Aryc at level 1
        And we have 1 Unrest
        And a die roll of 5
        When I Improve Lifestyle
        #Then there is a +1 circumstance bonus to Culture until the end of the turn, because "failure improving lifestyle"  //FIXME
        Then Unrest is still 1

