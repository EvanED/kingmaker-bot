Feature: Activity Phase, Step 2 (Region) -- Establish Work Site

# Your hire a crew of workers to travel to a hex that
# contains Lumber, Ore, or Stone to be harvested.
#
# * Spend RP as determined by the hex's most inhospitable
#   terrain (see the Building on Rough Terrain sidebar on
#   page 519).
#
# Then attempt a basic check. Lumber camps can be established
# in any hex that contains a significant amount of forest
# terrain. Mines and quarries can be established in any hex
# that contains a significant amount of hill or mountain terrain.

    # Critical Success
    #
    # You establish a Work Site in the hex and proceed to
    # discover an unexpectedly rich supply of high quality
    # Commodities. All Commodity yields granted by this site
    # are doubled until the end of the next Kingdom turn.
    Scenario: Establish Work Site on plains critically succeeds
        Given the kingdom Aryc at level 1
        And the kingdom has 15 RP
        And we have 1 Unrest
        And a die roll of 20
        When I Establish Work Site on plains
        Then there are 3 requirements
        And "mark the map with the new work site" is a requirement
        And "increase the income for the commodity by 1" is a requirement
        And "there is a bonus commodity income next turn" is a requirement
        And RP went down to 14
        And Unrest is still 1

    Scenario: Establish Work Site on hills critically succeeds
        Given the kingdom Aryc at level 1
        And the kingdom has 15 RP
        And we have 1 Unrest
        And a die roll of 20
        When I Establish Work Site on hills
        Then there are 3 requirements
        And "mark the map with the new work site" is a requirement
        And "increase the income for the commodity by 1" is a requirement
        And "there is a bonus commodity income next turn" is a requirement
        And RP went down to 13
        And Unrest is still 1

    # Success
    #
    # You establish a Work Site in the hex.
    Scenario: Establish Work Site succeeds
        Given the kingdom Aryc at level 1
        And the kingdom has 15 RP
        And we have 1 Unrest
        And a die roll of 10
        When I Establish Work Site on hills
        Then there are 2 requirements
        And "increase the income for the commodity by 1" is a requirement
        And "mark the map with the new work site" is a requirement
        And RP went down to 13
        And Unrest is still 1

    # Failure
    #
    # You fail to establish a Work Site in the hex.
    Scenario: Establish Work Site fails
        Given the kingdom Aryc at level 1
        And the kingdom has 15 RP
        And we have 1 Unrest
        And a die roll of 5
        When I Establish Work Site on hills
        Then there are no requirements
        And RP went down to 13
        And Unrest is still 1

    # Critical Failure
    #
    # Not only do you fail to establish a Work Site, but
    # you lose several workers to an accident, banditry,
    # a vicious monster, or some other unforeseen occurrence.
    # Gain 1 Unrest.
    Scenario: Establish Work Site critically fails
        Given the kingdom Aryc at level 1
        And the kingdom has 15 RP
        And we have 1 Unrest
        And a die roll of 1
        When I Establish Work Site on hills
        Then there are no requirements
        And RP went down to 13
        And Unrest went up by 2
