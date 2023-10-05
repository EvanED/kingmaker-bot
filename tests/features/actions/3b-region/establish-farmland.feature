Feature: Activity Phase, Step 2 (Region) -- Establish Farmland

# You plant crops and establish livestock in permanent farms,
# ranches, and other growing operations to create Farmland.
#
# If you're attempting to Establish Farmland in a hex that is
# predominantly plains, you must spend 1 RP and the check is
# against your Control DC. If you're targeting a hex that is
# predominantly hills, you must spend 2 RP and the check is
# against your Control DC + 5.
#
# Critical Success
#     You establish two adjacent Farmland hexes instead of
#     one. If your target hex was a hills hex, the additional
#     hex may be a hills hex or a plains hex; otherwise, the
#     additional hex must be a plains hex. If no appropriate
#     hex is available, treat this result as a regular success
#     instead.
#
# Success
#     You establish one Farmland hex.
#
# Failure
#     You fail to establish a Farmland hex.
#
# Critical Failure
#     You fail to establish a Farmland hex, and your attempt
#     potentially causes the spread of a blight. At the start
#     of each of the next two Event phases, attempt a DC 6
#     flat check; on a failure, your kingdom experiences a
#     Crop Failure event in this and all adjacent hexes.

    Scenario: Establish Farmland on plains critically succeeds
        Given the kingdom Aryc at level 1
        And a die roll of 20
        When I Establish Farmland on plains
        Then there are 2 requirements
        And "mark the map with the new farmland" is a requirement
        And "may immediately attempt Establish Farmland again (plains only)" is a requirement

    Scenario: Establish Farmland on hills critically succeeds
        Given the kingdom Aryc at level 1
        And a die roll of 20
        When I Establish Farmland on hills
        Then there are 2 requirements
        And "mark the map with the new farmland" is a requirement
        And "may immediately attempt Establish Farmland again (plains or hills)" is a requirement

    Scenario: Establish Farmland succeeds
        Given the kingdom Aryc at level 1
        And a die roll of 19
        When I Establish Farmland on hills
        Then there is 1 requirement
        And "mark the map with the new farmland" is a requirement

    Scenario: Establish Farmland fails
        Given the kingdom Aryc at level 1
        And a die roll of 5
        When I Establish Farmland on hills
        Then there are no requirements

    Scenario: Establish Farmland critically fails
        Given the kingdom Aryc at level 1
        And a die roll of 1
        When I Establish Farmland on hills
        Then there are no requirements
        And there is a Crop Failure potential for next 2 turns
