# Magic Math in Rust

This is an implementation of various math and probability theory written in 
[Rust](http://www.rust-lang.org).

## Quick start

You should be able to compile the program using

	$ make run

Or, if using windows

	$ make -f Makefile-win

## The Rules of the Game

We try to figure out how to design a deck such that you have the best
chance of being to cast your spells. In Magic, you design your own 40-card or 60-card
deck. Cards in your deck are either a land or a spell.

You start the game by shuffling your deck, drawing 7 cards. If you like your hand, you 
keep it, but if you don't you can shuffle the deck again, drawing 6 cards. You repeat
this process until you are happy. This is called "mulligan".

You can play one land per turn. This makes mana available for you to cast your spells. 
The mana-cost is different depending on the spell. This means that you can only play
spells with cost 1 on turn 1, 2 on turn 2 and so on, provided that you have land in 
your hand.

For each turn (except the first if you are on the play), you draw a card from your deck.

Spell cost is either colored or not. There are five colors in magic, (white, blue, black,
red and green). A spell that is 1W, for instance requires one white mana and one mana of 
any color. There are different land-types too. Basic lands produce one mana of its own 
color (plains => white, island => blue, swamp => black, mountain => red, forest => green);
some only produce colorless mana; some lands can produce one of two colors (dual-lands); 
and some may produce any color. 

You may only put up to 4 cards of the same type in your deck, except for basic lands,
you can put any number of those in your deck.
