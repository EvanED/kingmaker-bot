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
        
    Scenario: Supernatural Solution success
    Scenario: Supernatural Solution failure
    Scenario: Supernatural Solution critical failure
