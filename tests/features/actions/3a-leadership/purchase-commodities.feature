Feature: Activity Phase, Step 1 (Leadership) -- Purchase Commodities

    Scenario: Purchase Commodities critically succeeds (non-luxury)
        Given the kingdom Aryc at level 1
        And the kingdom has 6 RP
        And the kingdom has 1 Food
        And the kingdom has 1 Lumber
        And a die roll of 20
        When I Purchase Commodities of Food (secondary Lumber)
        Then RP went down to 2
        And I have 5 Food
        And I have 3 Lumber

    Scenario: Purchase Commodities succeeds (non-luxury)
        Given the kingdom Aryc at level 1
        And the kingdom has 6 RP
        And the kingdom has 1 Food
        And the kingdom has 1 Lumber
        And a die roll of 15
        When I Purchase Commodities of Food (secondary Lumber)
        Then RP went down to 2
        And I have 3 Food
        And I have 1 Lumber

    Scenario: Purchase Commodities fails (non-luxury)
        Given the kingdom Aryc at level 1
        And the kingdom has 6 RP
        And the kingdom has 1 Food
        And the kingdom has 1 Lumber
        And a die roll of 5
        When I Purchase Commodities of Food (secondary Lumber)
        Then RP went down to 2
        And I have 2 Food
        And I have 1 Lumber

    Scenario: Purchase Commodities critically fails (non-luxury)
        Given the kingdom Aryc at level 1
        And the kingdom has 6 RP
        And the kingdom has 1 Food
        And the kingdom has 1 Lumber
        And a die roll of 1
        When I Purchase Commodities of Food (secondary Lumber)
        Then RP went down to 2
        And I have 1 Food
        And I have 1 Lumber

    Scenario: Purchase Commodities of Luxuries critically succeeds
        Given the kingdom Aryc at level 1
        And the kingdom has 10 RP
        And the kingdom has 1 Luxury
        And the kingdom has 1 Lumber
        And a die roll of 20
        When I Purchase Commodities of Luxuries (secondary Lumber)
        Then RP went down to 2
        And I have 5 Luxuries
        And I have 3 Lumber

# TODO: Select secondary option later
