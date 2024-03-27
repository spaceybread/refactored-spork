Busy Beaver game: an attempt to find a program that eventually halts while producing the largest output

[beaver32.rs](https://github.com/spaceybread/refactored-spork/blob/main/beaver32.rs): is a 3-state, 2-symbol busy beaver which is known to have an output of 6. Formally, one might write it as $BB(3, 2) = \Sigma(3, 2) = 6$

[beaver42.rs](https://github.com/spaceybread/refactored-spork/blob/main/beaver42.rs): 4-state, 2-symbol: $BB(4, 2) = \Sigma(4, 2) = 13$

[state_beaver.rs](https://github.com/spaceybread/refactored-spork/blob/main/state_beaver.rs): utilises vectors for the change of state function instead of using if/else statements; allows for easy maninpulation and expansion
