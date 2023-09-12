Feature: Commerce Phase, Step 3 -- Improve Lifestyle

    Scenario: Trade Commodities critically succeeds
        Given the kingdom Aryc at level 1
        And we have 3 Lumber
        And a die roll of 20
        And next turn will have 1 bonus RP
        When I Trade Commodities of 2 Lumber
        Then next turn will have 5 bonus RP
        And I have 1 Lumber
        And we traded commodities this turn

    Scenario: Trade Commodities suceeds
        Given the kingdom Aryc at level 1
        And we have 3 Lumber
        And a die roll of 15
        And next turn will have 1 bonus RP
        When I Trade Commodities of 2 Lumber
        Then next turn will have 3 bonus RP
        And I have 1 Lumber
        And we traded commodities this turn

    Scenario: Trade Commodities fails
        Given the kingdom Aryc at level 1
        And we have 3 Lumber
        And a die roll of 5
        And next turn will have 1 bonus RP
        When I Trade Commodities of 2 Lumber
        Then next turn will have 2 bonus RP
        And I have 1 Lumber
        And we traded commodities this turn

    Scenario: Trade Commodities critically fails, without having traded last turn
        Given the kingdom Aryc at level 1
        And we have 3 Lumber
        And a die roll of 1
        And next turn will have 1 bonus RP
        And we have 1 Unrest
        When I Trade Commodities of 2 Lumber
        Then next turn will have 1 bonus RP
        And I have 1 Lumber
        And we traded commodities this turn
        And Unrest is still 1

    Scenario: Trade Commodities critically fails, having traded last turn
        Given the kingdom Aryc at level 1
        And we have 3 Lumber
        And a die roll of 1
        And next turn will have 1 bonus RP
        And the kingdom traded commodities the previous turn
        And we have 1 Unrest
        When I Trade Commodities of 2 Lumber
        Then next turn will have 1 bonus RP
        And I have 1 Lumber
        And we traded commodities this turn
        And Unrest went up to 2

# TODO: Scenarios for the +1 circum bonus