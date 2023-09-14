Feature: Activity Phase, Step 2 (Region) -- Go Fishing

# Attempt a basic check to fish for food
# from the rivers and lakes in your kingdom.

# Critical Success
#     Gain 1d4 Food commodities.
#
# Success
#     Gain 1 Food commodity.
#
# Failure
#     Gain no Food commodities.
#
# Critical Failure
#     You lose some fishers to tragic accidents; gain 1 Unrest.

    Scenario: Go Fishing critically succeeds
        Given the kingdom Aryc at level 1
        And the kingdom has 1 Food
        And we have 1 Unrest
        And a die roll of 20
        And the next d4 rolls are 3
        When I Go Fishing
        Then the kingdom's Food went up to 4
        And Unrest is still 1

    Scenario: Go Fishing succeeds
        Given the kingdom Aryc at level 1
        And the kingdom has 1 Food
        And we have 1 Unrest
        And a die roll of 15
        And the next d4 rolls are 3
        When I Go Fishing
        Then the kingdom's Food went up to 2
        And Unrest is still 1

    Scenario: Go Fishing fails
        Given the kingdom Aryc at level 1
        And the kingdom has 1 Food
        And we have 1 Unrest
        And a die roll of 5
        And the next d4 rolls are 3
        When I Go Fishing
        Then the kingdom's Food is still 1
        And Unrest is still 1

    Scenario: Go Fishing critically fails
        Given the kingdom Aryc at level 1
        And the kingdom has 1 Food
        And we have 1 Unrest
        And a die roll of 1
        And the next d4 rolls are 3
        When I Go Fishing
        Then the kingdom's Food is still 1
        And Unrest went up to 4
