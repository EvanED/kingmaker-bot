Feature: Activity Phase, Step 2 (Region) -- Claim Hex

# Your surveyors fully explore the hex and attempt to add it
# into your kingdom's domain. Spend 1 RP and then attempt a
# basic Exploration, Intrigue, Magic, or Wilderness check.
#
#
# Critical Success
#     You claim the hex and immediately add it to your
#     territory, increasing your kingdom's Size by 1
#     (this affects all statistics determined by Size;
#     see page 532).
#
#     Your occupation of the hex goes so smoothly that
#     you can immediately attempt another Region activity.

    # TODO: Size tracking
    # TODO: Better turn tracking

    Scenario: Claim Hex critically succeeds
        Given the kingdom Aryc at level 1
        And the kingdom has 2 RP
        And a die roll of 20
        When I Claim Hex with Magic
        Then there are 3 requirements
        And "mark the hex 0.0 as claimed" is a requirement
        And "gain XP from the claimed hex" is a requirement
        And "you may take another region activity" is a requirement
        And RP went down to 1

# Success
#     You claim the hex and add it to your territory,
#     increasing your kingdom's Size by 1 (this affects
#     all statistics determined by Size; see page 532).

    Scenario: Claim Hex succeeds
        Given the kingdom Aryc at level 1
        And the kingdom has 2 RP
        And a die roll of 15
        When I Claim Hex with Magic
        Then there are 2 requirements
        And "mark the hex 0.0 as claimed" is a requirement
        And "gain XP from the claimed hex" is a requirement
        And RP went down to 1

# Failure
#     You fail to claim the hex.

    Scenario: Claim Hex fails
        Given the kingdom Aryc at level 1
        And the kingdom has 2 RP
        And a die roll of 5
        When I Claim Hex with Magic
        Then there are no requirements
        And RP went down to 1

# Critical Failure
#     You fail to claim the hex, and a number of early
#     settlers and explorers are lost, causing you to
#     take a –1 circumstance penalty to Stability-based
#      checks until the end of your next Kingdom turn.

    Scenario: Claim Hex critically fails
        Given the kingdom Aryc at level 1
        And the kingdom has 2 RP
        And a die roll of 1
        When I Claim Hex with Magic
        Then there are no requirements
        And RP went down to 1
        And there is a -1 circumstance penalty to Stability until the end of the next turn, because "critical failure in Claim Hex"

# Special:
#
# At 1st level, when selecting the three activities you take
# during the Region Activities step of the Activity phase of
# the Kingdom turn, you may select this activity no more than
# once.
#
# Once your kingdom reaches 4th level, you may select it up to
# twice per turn, and after reaching 9th level you may select it
# up to three times per turn.
#
# When you successfully claim a hex, gain 10 kingdom XP (see
# Hex Claim XP Awards). Many hexes have terrain features that
# grant benefits to your kingdom when claimed; see Terrain Features.
