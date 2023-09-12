Feature: Commerce Phase, Step 3 -- Improve Lifestyle

    Scenario: Trade Commodities critically succeeds
        Given the kingdom Aryc at level 1
        And we have 3 Lumber
        And a die roll of 20
        And next turn will have 1 bonus RP
        When I Trade Commodities of 2 Lumber
        Then next turn will have 4 bonus RP

    Scenario: Trade Commodities suceeds
    Scenario: Trade Commodities fails
    Scenario: Trade Commodities critically fails, without having traded last turn
    Scenario: Trade Commodities critically fails, having traded last turn

# TODO: Scenarios for the +1 circum bonus