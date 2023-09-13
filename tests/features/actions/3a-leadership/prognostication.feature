Feature: Activity Phase, Step 1 (Leadership) -- Prognostication

    # TODO: Multiple attempts

    Scenario: Prognostication critically succeeds
        Given the kingdom Aryc at level 1
        And random kingdom event selection will be normal
        And a die roll of 20
        When I Prognosticate
        Then there is a +2 circumstance bonus to The Event until the end of the turn, because "critical success on Prognostication"
        And the players have advantage on selection of a random kingdom event this turn

    Scenario: Prognostication succeeds
        Given the kingdom Aryc at level 1
        And random kingdom event selection will be normal
        And a die roll of 15
        When I Prognosticate
        Then there is a +1 circumstance bonus to The Event until the end of the turn, because "critical success on Prognostication"
        And random kingdom event selection is normal

    Scenario: Prognostication fails
        Given the kingdom Aryc at level 1
        And random kingdom event selection will be normal
        And a die roll of 5
        When I Prognosticate
        Then there is no bonus
        And random kingdom event selection is normal

    Scenario: Prognostication critically fails
        Given the kingdom Aryc at level 1
        And random kingdom event selection will be normal
        And a die roll of 1
        When I Prognosticate
        Then there is no bonus
        And the GM has advantage on selection of a random kingdom event this turn
