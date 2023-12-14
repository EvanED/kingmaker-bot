Feature: Activity Phase, Step 2 (Region) -- Build Roads

    Scenario: Build Roads on plains critically succeeds
        Given the kingdom Aryc at level 1
        And the kingdom has 15 RP
        And we have 1 Unrest
        And a die roll of 20
        When I Build Roads on plains
        Then there are 3 requirements
        And "mark the map with the new roads" is a requirement
        And "can mark an adjacent hex with new roads as well, if it is plains or easier" is a requirement
        And "subtract 1 RP if there is a river crossing the hex"
        And RP went down to 14
        And Unrest is still 1

    Scenario: Build Roads on mountains critically succeeds
        Given the kingdom Aryc at level 1
        And the kingdom has 15 RP
        And we have 1 Unrest
        And a die roll of 20
        When I Build Roads on mountains
        Then there are 3 requirements
        And "mark the map with the new roads" is a requirement
        And "can mark an adjacent hex with new roads as well, if it is mountains or easier" is a requirement
        And "subtract 12 RP if there is a river crossing the hex"
        And RP went down 3
        And Unrest is still 1

    Scenario: Build Roads succeeds
        Given the kingdom Aryc at level 1
        And the kingdom has 15 RP
        And we have 1 Unrest
        And a die roll of 19
        When I Build Roads on hills
        Then there are 2 requirements
        And "mark the map with the new roads" is a requirement
        And "subtract 2 RP if there is a river crossing the hex"
        And RP went down to 13
        And Unrest is still 1

    Scenario: Build Roads fails
        Given the kingdom Aryc at level 1
        And the kingdom has 15 RP
        And we have 1 Unrest
        And a die roll of 5
        When I Build Roads on forest
        Then there are no requirements
        And RP went down to 11
        And Unrest is still 1
        And there is 1 requirement
        And "subtract 4 RP if there is a river crossing the hex"

    Scenario: Build Roads critically fails
        Given the kingdom Aryc at level 1
        And the kingdom has 15 RP
        And we have 1 Unrest
        And a die roll of 1
        When I Build Roads on swamp
        Then there are no requirements
        And RP went down to 7
        And Unrest went up to 2
        And there is 1 requirement
        And "subtract 8 RP if there is a river crossing the hex"
