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

    Scenario: Claim Hex critically succeds
        Given the kingdom Aryc at level 1
        And the kingdom has 2 RP
        And a die roll of 20
        When I Claim Hex with Magic
        Then there are 2 requirements
        And "mark the hex as claimed" is a requirement
        And "you may take another region activity" is a requirement
        And RP went down to 1

#
# Success
#     You claim the hex and add it to your territory,
#     increasing your kingdom's Size by 1 (this affects
#     all statistics determined by Size; see page 532).
#
# Failure
#     You fail to claim the hex.
#
# Critical Failure
#     You fail to claim the hex, and a number of early
#     settlers and explorers are lost, causing you to
#     take a –1 circumstance penalty to Stability-based
#      checks until the end of your next Kingdom turn.
#
#
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
