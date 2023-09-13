Feature: Create a Masterpiece

    Scenario: Create a Masterpiece critically succeeds
        Given the kingdom Aryc at level 1
        And the kingdom has not Created a Masterpiece this turn
        And the kingdom has 1 Fame point
        And the kingdom is not scheduled to gain a Fame point at the start of next turn
        And the kingdom has 1 RP
        And we have 1 Unrest
        And a die roll of 20
        And the next d4 rolls are 3
        When I Create a Masterpiece
        Then Fame points went up to 2
        And the kingdom will gain one additional Fame point next turn
        And RP went up to 7
        And Unrest is still 1

    Scenario: Create a Masterpiece succeeds
        Given the kingdom Aryc at level 1
        And the kingdom has not Created a Masterpiece this turn
        And the kingdom has 1 Fame point
        And the kingdom is not scheduled to gain a Fame point at the start of next turn
        And the kingdom has 1 RP
        And we have 1 Unrest
        And a die roll of 15
        When I Create a Masterpiece
        Then Fame points went up to 2
        And the kingdom will not gain one additional Fame point next turn
        And RP is still 1
        And Unrest is still 1

    Scenario: Create a Masterpiece fails
        Given the kingdom Aryc at level 1
        And the kingdom has not Created a Masterpiece this turn
        And the kingdom has 1 Fame point
        And the kingdom is not scheduled to gain a Fame point at the start of next turn
        And the kingdom has 1 RP
        And we have 1 Unrest
        And a die roll of 5
        When I Create a Masterpiece
        Then Fame points is still 1
        And the kingdom will not gain one additional Fame point next turn
        And RP is still 1
        And Unrest is still 1

    Scenario: Create a Masterpiece critically fails
        Given the kingdom Aryc at level 1
        And the kingdom has not Created a Masterpiece this turn
        And the kingdom has 2 Fame points
        And the kingdom is not scheduled to gain a Fame point at the start of next turn
        And the kingdom has 1 RP
        And we have 1 Unrest
        And a die roll of 1
        When I Create a Masterpiece
        Then Fame points went down to 1
        And the kingdom will not gain one additional Fame point next turn
        And RP is still 1
        And Unrest is still 1

    Scenario: Create a Masterpiece critically fails, and the Fame cost is unaffordable
        Given the kingdom Aryc at level 1
        And the kingdom has not Created a Masterpiece this turn
        And the kingdom has 0 Fame points
        And the kingdom is not scheduled to gain a Fame point at the start of next turn
        And the kingdom has 1 RP
        And we have 1 Unrest
        And a die roll of 1
        And the next d4 rolls are 3
        When I Create a Masterpiece
        Then Fame points is still 0
        And the kingdom will not gain one additional Fame point next turn
        And RP is still 1
        And Unrest went up to 4
