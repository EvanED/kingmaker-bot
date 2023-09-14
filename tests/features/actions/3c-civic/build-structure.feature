Feature: Activity Phase, Step 3 (Civic) -- Build Structure

# You attempt to build a structure in the settlement that's
# granting the Civic activity. You may choose any structure
# for which you meet the requirements. Select the appropriate
# number of contiguous buildable lots in a single block as
# specified by the structure's entry and spend the specified
# RP and Commodity cost. Then attempt the structure's skill
# check.
#
# You can also use this activity to attempt to repair a
# structure that was damaged as the result of an event but
# hasn't been replaced by Rubble. To do this, first spend
# half the structure's listed RP and Commodity cost, and
# then attempt the specified check. The existing structure
# gives you a +2 item bonus to the check.
#
# On a success, record the new construction on the Urban
# Grid. Unless the structure's entry states otherwise,
# its effects are immediate; if the structure adjusts a
# Ruin's point total, adjust it upon construction.

# Critical Success
#     You construct or repair the structure with great
#     efficiency and get back half of the Commodities
#     spent in construction or repair.

    # TODO handle "others" Commodity cost...

    Scenario: Build Structure critically succeeds
        Given the kingdom Aryc at level 1
        And the kingdom has 10 RP
        And the kingdom has 5 Stone
        And a die roll of 20
        When I Build a Shrine Structure 
        Then there are 2 requirements
        And "mark the urban grid with the new stucture" is a requirement
        And "adjust kingdom item bonuses accordingly" is a requirement
        And RP went down to 2
        And the kingdom's Stone went down to 3
        And next turn can not re-attempt building a Shrine at no resource cost

# Success
#     You construct or repair the structure.

    Scenario: Build Structure critically succeeds
        Given the kingdom Aryc at level 1
        And the kingdom has 10 RP
        And the kingdom has 5 Stone
        And a die roll of 15
        When I Build a Shrine Structure 
        Then there are 2 requirements
        And "mark the urban grid with the new stucture" is a requirement
        And "adjust kingdom item bonuses accordingly" is a requirement
        And RP went down to 2
        And the kingdom's Stone went down to 2
        And next turn can not re-attempt building a Shrine at no resource cost

# Failure
#     You fail to construct or repair the structure.
#     You can try to complete it next Kingdom turn; if
#     you do so, you do not need to re-pay the RP and
#     Commodity cost.

    Scenario: Build Structure fails
        Given the kingdom Aryc at level 1
        And the kingdom has 10 RP
        And the kingdom has 5 Stone
        And a die roll of 5
        When I Build a Shrine Structure 
        Then RP went down to 2
        And the kingdom's Stone went down to 2
        And next turn can re-attempt building a Shrine at no resource cost

# Critical Failure
#     You fail to construct the structure; if you were
#     attempting to repair a damaged structure, it is
#     reduced to Rubble.
#
#     In either event, Rubble now fills the structure's
#     lots, which must be cleared with the Demolish
#     activity before you can attempt to Build a
#     Structure in them again.

    Scenario: Build Structure critically fails
        Given the kingdom Aryc at level 1
        And the kingdom has 10 RP
        And the kingdom has 5 Stone
        And a die roll of 1
        When I Build a Shrine Structure 
        Then there is 1 requirement
        And "fill the lot(s) in the Urban Grid with rubble" is a requirement
        And RP went down to 2
        And the kingdom's Stone went down to 2
        And next turn can not re-attempt building a Shrine at no resource cost
