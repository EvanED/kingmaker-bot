Feature: Supernatural Solution

    # TODO: what happens in failure-to-pay cases?

    Scenario: Supernatural Solution critical success
        Given the kingdom Aryc at level 1
        And the Supernatural Solution is not available
        And the kingdom has 3 RP
        And a die roll of 20
        When I search for a Supernatural Solution
        Then the Supernatural Solution fortune is available
        And RP is still 3
        And Supernatural Solution is not blocked
        
    Scenario: Supernatural Solution success
        Given the kingdom Aryc at level 1
        And the Supernatural Solution is not available
        And the kingdom has 3 RP
        And a die roll of 15
        And the next d4 rolls are 2
        When I search for a Supernatural Solution
        Then the Supernatural Solution fortune is available
        And RP went down to 1
        And Supernatural Solution is not blocked
        
    Scenario: Supernatural Solution failure
        Given the kingdom Aryc at level 1
        And the Supernatural Solution is not available
        And the kingdom has 13 RP
        And a die roll of 5
        And the next d6 rolls are 5
        When I search for a Supernatural Solution
        Then the Supernatural Solution fortune is not available
        And RP went down to 3
        And Supernatural Solution is not blocked

    Scenario: Supernatural Solution critical failure
        Given the kingdom Aryc at level 1
        And the Supernatural Solution is not available
        And the kingdom has 13 RP
        And a die roll of 1
        And the next d6 rolls are 5
        When I search for a Supernatural Solution
        Then the Supernatural Solution fortune is not available
        And RP went down to 3
        And Supernatural Solution is blocked for two turns
