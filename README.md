# Chicago-Card-Game

A command line based card game that I made for two classes, software engineering and organization of programming languages

Basically it is like Rummy 500
  * Runs
      * Any 4 or more cards in a row that share the same color and suit
    Ex. 2, 3, 4, 5 of spades in red
  Set
    Any 3 or more cards that are the same rank, can be any color
    Ex. 3, 3, 3 of spades, hearts, in red and black
Goal is to go out
To go out you must have no cards in your hand after discarding your card

A round consists of 3, or 5 players with 11 cards, the first person up picks up a card, and if they meet the requirements for putting down for that specific round they can do so, otherwise you have to discard a card.
Next, someone has the option to “May I”, which means that if someone who isn’t up wants the card that was discarded, everyone has to agree to it and if so that person gets the card and a card from the shuffled deck, you can only have 3 May I’s a round
6 Rounds:
  2 sets
  2 runs
  2 set and a run
  2 runs and a set
  3 sets
  3 runs (you must use at least one May I)

Points:
  If you have cards in your hand if someone goes out
    2 - 9 = 5 pts
    10, Jack, Queen, King = 10 pts
    Ace = 15 pts
    Joker = 50 pts
A joker can be any card you want, think of it as a free card, except if you get stuck with one in your hand it can cost you.

The demo is version 1.0 of my card game, fully playable for 3 players
  There are a few errors that may pop up if someone decides to enter the wrong card name for example.
  Mostly it is error handling that is left, but for the purpose of the other class this code meets the requirements
  Command Line based
  Basis for my senior project
This demo does not have the cards shuffled as it would take a long time to actually show off a full game as it can take anywhere from 45 minutes onward if playing with real cards
