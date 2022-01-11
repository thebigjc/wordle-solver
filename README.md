# wordle solver

Reads words from legal.txt and words2.txt. legal.txt is the full list of words that can be guessed, and words2.txt is the list of words that could be the right answer.

Uses rayon simply for fun since much of this work is naive parallizable.

## How it works

We calculate the entropy for each legal word against the possible outcomes and how many words fall into each possible class. Based on this, we guess the word that has the highest entropy, which is effectively the highest information gain. 

There is no setup for incorporating feedback and making additional guesses. In order to do that, change words2.txt to have less words based on the feedback (I use use a chain of greps). 

What isn't clear is when to guess a word that adds information that can't be the right answer, or guessing a word that could be the right answer but adds less information. That remains an open proble IMHO that I'd love some thoughts on.

